#![no_std]
#![no_main]

use core::panic::PanicInfo;
use unoperating_system::*;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{_info}");
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    write_start_text();

    print!("{}", 'H');
    print!("ello! ");
    println!("The numbers are {} and {}", 42, 1.0/3.0);
    
    panic!("panic attack");
    
    loop {}
}

fn write_start_text() {
    println!("===========================\\");
    println!("  _   _        ___  ____   |");
    println!(" | | | |_ __  / _ \\/ ___|  |");
    println!(" | | | | '_ \\| | | \\___ \\  |");
    println!(" | |_| | | | | |_| |___) | |");
    println!("  \\___/|_| |_|\\___/|____/  |");
    println!("                           |");
    println!("  v0.1.0 alpha             |");
    println!("===========================/");
}
