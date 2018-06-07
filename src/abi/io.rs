use alloc::String;
use x86_64::instructions::port::Port;
use nebulet_derive::nebulet_abi;
use wasm::instance::VmCtx;
use object::ProcessRef;

#[nebulet_abi]
pub fn print(buffer_offset: u32, buffer_size: u32, process: &ProcessRef) {
    let instance = process.instance().read();
    let memory = &instance.memories[0];
    if let Some(buf) = memory.carve_slice(buffer_offset, buffer_size) {
        let s = String::from_utf8_lossy(buf);
        println!("{}", s);
    }
    else {
        println!("\nPrinting invalid buffer!")
    }
}

pub unsafe fn read_port_u8(port: u32, _vmctx: &VmCtx) -> u32 {
    let port = port as u16;
    let ret = Port::<u8>::new(port).read();
    ret as u32
}

pub unsafe fn write_port_u8(port: u32, val: u32, _vmctx: &VmCtx) {
    let port = port as u16;
    let val = val as u8;
    Port::<u8>::new(port).write(val)
}