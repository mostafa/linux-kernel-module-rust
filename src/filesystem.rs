use alloc::boxed::Box;
use core::default::Default;
use core::marker;

use bitflags;

use crate::bindings;
use crate::c_types;
use crate::error;
use crate::types::CStr;

pub struct FileSystemRegistration<T: FileSystem> {
    _phantom: marker::PhantomData<T>,
    ptr: Box<bindings::file_system_type>,
}

impl<T: FileSystem> Drop for FileSystemRegistration<T> {
    fn drop(&mut self) {
        unsafe { bindings::unregister_filesystem(&mut *self.ptr) };
    }
}

pub trait FileSystem {
    const NAME: &'static CStr;
    const FLAGS: FileSystemFlags;
}

bitflags::bitflags! {
    pub struct FileSystemFlags: c_types::c_int {
        const FS_REQUIRES_DEV = bindings::FS_REQUIRES_DEV as c_types::c_int;
        const FS_BINARY_MOUNTDATA = bindings::FS_BINARY_MOUNTDATA as c_types::c_int;
        const FS_HAS_SUBTYPE = bindings::FS_HAS_SUBTYPE as c_types::c_int;
        const FS_USERNS_MOUNT = bindings::FS_USERNS_MOUNT as c_types::c_int;
        const FS_RENAME_DOES_D_MOVE = bindings::FS_RENAME_DOES_D_MOVE as c_types::c_int;
    }
}

extern "C" fn fill_super_callback<T: FileSystem>(
    _sb: *mut bindings::super_block,
    _data: *mut c_types::c_void,
    _silent: c_types::c_int,
) -> c_types::c_int {
    // T::fill_super(...)
    // This should actually create an object that gets dropped by
    // file_system_registration::kill_sb. You can point to it with
    // sb->s_fs_info.
    unimplemented!();
}

extern "C" fn mount_callback<T: FileSystem>(
    fs_type: *mut bindings::file_system_type,
    flags: c_types::c_int,
    _dev_name: *const c_types::c_char,
    data: *mut c_types::c_void,
) -> *mut bindings::dentry {
    unsafe { bindings::mount_nodev(fs_type, flags, data, Some(fill_super_callback::<T>)) }
}

pub fn register<T: FileSystem>() -> error::KernelResult<FileSystemRegistration<T>> {
    let mut fs_registration = FileSystemRegistration {
        ptr: Box::new(bindings::file_system_type {
            name: T::NAME.as_ptr() as *const i8,
            owner: unsafe { &mut bindings::__this_module },
            fs_flags: T::FLAGS.bits(),
            mount: Some(mount_callback::<T>),
            kill_sb: Some(bindings::kill_litter_super),

            ..Default::default()
        }),
        _phantom: marker::PhantomData,
    };
    let result = unsafe { bindings::register_filesystem(&mut *fs_registration.ptr) };
    if result != 0 {
        return Err(error::Error::from_kernel_errno(result));
    }

    Ok(fs_registration)
}
