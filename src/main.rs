#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::fmt::*;
use unoperating_system::console::*;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    CONSOLE.lock().write_str("===========================\\\n");
    CONSOLE.lock().write_str("  _   _        ___  ____   |\n");
    CONSOLE.lock().write_str(" | | | |_ __  / _ \\/ ___|  |\n");
    CONSOLE.lock().write_str(" | | | | '_ \\| | | \\___ \\  |\n");
    CONSOLE.lock().write_str(" | |_| | | | | |_| |___) | |\n");
    CONSOLE.lock().write_str("  \\___/|_| |_|\\___/|____/  |\n");
    CONSOLE.lock().write_str("                           |\n");
    CONSOLE.lock().write_str("  v0.1.0 alpha             |\n");
    CONSOLE.lock().write_str("===========================/\n");

    CONSOLE.lock().write_char('H');
    CONSOLE.lock().write_str("ello! ");
    write!(CONSOLE.lock(), "The numbers are {} and {}", 42, 1.0/3.0).unwrap();
    loop {}
}
