#![no_std]
#![no_main]

use core::panic::PanicInfo;

const UCSR0A: *mut u8 = 0xc0 as *mut u8;
const UCSR0B: *mut u8 = 0xc1 as *mut u8;
const UCSR0C: *mut u8 = 0xc2 as *mut u8;

const UBBR0L: *mut u8 = 0xc4 as *mut u8;
const UBBR0H: *mut u8 = 0xc5 as *mut u8;

const UDR0: *mut u8 = 0xc6 as *mut u8;

const UDRE0: u8 = 5;
const TXEN0: u8 = 3;
const UCSZ01: u8 = 2;
const UCSZ00: u8 = 1;
const U2X0: u8 = 1;

const F_CPU: u32 = 16_000_000;
const BAUD: u32 = 115_200;
const UBRR: u16 = ((F_CPU / (BAUD * 8)) - 1) as u16;

fn uart_init() {
    unsafe {
        core::ptr::write_volatile(UCSR0A, 1 << U2X0);
        
        core::ptr::write_volatile(UBBR0H, (UBRR >> 8) as u8);

        core::ptr::write_volatile(UBBR0L, UBRR as u8);

        core::ptr::write_volatile(UCSR0B, 1 << TXEN0);
        core::ptr::write_volatile(UCSR0C, (1 << UCSZ01) | (1 << UCSZ00));
    }
}

fn uart_write(byte: u8) {
    unsafe {
        while core::ptr::read_volatile(UCSR0A) & (1 << UDRE0) == 0 {}
        core::ptr::write_volatile(UDR0, byte);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    uart_init();

    let string = b"Hello World from ATmega328P!\r\n";
    for &byte in string {
        uart_write(byte);
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
