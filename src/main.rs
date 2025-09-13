#![no_std] // 不链接 Rust 标准库
#![no_main] // 禁用所有 Rust 层级的入口点
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use blog_os::println;
use core::panic::PanicInfo;

#[unsafe(no_mangle)] // 不重整函数名
pub extern "C" fn _start() -> ! {
    println!("Hello, world{}", "!");

    #[cfg(test)]
    test_main();

    loop {}
}

/// 这个函数将在 panic 时被调用
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

// our panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
}


#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}


