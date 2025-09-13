#![no_std] // 不链接 Rust 标准库
#![no_main] // 禁用所有 Rust 层级的入口点

use core::panic::PanicInfo;
mod vga_buffer;

/// 这个函数将在 panic 时被调用
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)] // 不重整函数名
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something(); 
    loop {}
}
