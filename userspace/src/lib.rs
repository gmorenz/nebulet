#![feature(
    wasm_import_module,
    global_allocator,
)]

extern crate wee_alloc;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub mod abi {
    #[wasm_import_module = "abi"]
    extern {
        pub fn print(ptr: *const u8, len: usize);
        pub fn set_idt_handler(index: u8, hander: unsafe extern "sysv64" fn());
        pub fn read_port_u8(port: u16) -> u8;
        pub fn write_port_u8(port: u16, val: u8);
    }
}

pub fn print(x: &str) {
    unsafe {
        abi::print(x.as_ptr(), x.len());
    }
}
