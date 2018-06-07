#![no_main]

extern crate userspace;

use userspace::{print, abi::{write_port_u8, read_port_u8, set_idt_handler}};

static MASTER_PIC_PORT: u16 = 0x20;

unsafe fn ack_master_pic() {
    write_port_u8(MASTER_PIC_PORT, 0x20)
}

pub unsafe extern "sysv64" fn handler() {
    let scancode = read_port_u8(0x60);
    print(&format!("Keyboard interupt: {}\n", scancode));

    ack_master_pic();
}

#[no_mangle]
pub fn main() {
    print("Initializing keyboard.");
    unsafe{ set_idt_handler(33, handler) };
    loop {}
}
