#![no_std] 
#![no_main] 

mod vga_buf;

use core::fmt::Write;
use core::panic::PanicInfo;

use crate::vga_buf::{Alignment, Color, Screen};


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle] 
pub extern "C" fn _start() -> ! { 

    
    let mut screen = Screen::new(Color::LIGHT_GREEN as u8, Alignment::Center);

    
    for i in 0..100 {
        write!(screen, "Number {}\n", i);
    }

    loop {}
}
