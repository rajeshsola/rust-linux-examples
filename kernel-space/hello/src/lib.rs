#![no_std]

use kernel::prelude::*;

module! {
    type: HelloModule,
    name: "hello_rust",
    author: "Rajesh Sola",
    description: "Simple Rust Kernel Module",
    license: "GPL",
}

struct HelloModule;

impl kernel::Module for HelloModule {
    fn init(_module:&'static ThisModule) -> Result<Self> {
        pr_info!("Hello from Rust kernel module!\n");
        Ok(HelloModule)
    }
}

impl Drop for HelloModule {
    fn drop(&mut self) {
        pr_info!("Goodbye from Rust kernel module!\n");
    }
}

// rust/kernel/lib.rs ==> pub trait Module
// core::ops::Drop 

