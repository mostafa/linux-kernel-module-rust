#![no_std]
#![feature(const_str_as_bytes)]

use linux_kernel_module::{self, println};

struct PrintkTestModule;

impl linux_kernel_module::KernelModule for PrintkTestModule {
    fn init() -> linux_kernel_module::KernelResult<Self> {
        println!("Single element printk");
        println!();
        println!("printk with {} parameters{}", 2, "!");

        Ok(PrintkTestModule)
    }
}

linux_kernel_module::kernel_module!(
    PrintkTestModule,
    author: "Alex Gaynor and Geoffrey Thomas",
    description: "A module for testing println!()",
    license: "GPL"
);
