#![no_std]
#![no_main]

use core::panic::PanicInfo;
use unoperating_system::console::*;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let mut con = Console::new();

    con.write_line("===========================\\");
    con.write_line("  _   _        ___  ____   |");
    con.write_line(" | | | |_ __  / _ \\/ ___|  |");
    con.write_line(" | | | | '_ \\| | | \\___ \\  |");
    con.write_line(" | |_| | | | | |_| |___) | |");
    con.write_line("  \\___/|_| |_|\\___/|____/  |");
    con.write_line("                           |");
    con.write_line("  v0.1.0 alpha             |");
    con.write_line("===========================/");

    loop {}
}
