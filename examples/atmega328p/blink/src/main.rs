#![no_std]
#![no_main]
#![deny(warnings)]

use rustduino::hal as arduino_uno;

#[no_mangle]
pub extern "C" fn main() {
    // Disable watchdog
    let wdog = arduino_uno::watchdog::Watchdog::new();
    wdog.disable();

    // Get all pins at once
    let mut pins = arduino_uno::pins::Pins::new();

    //Set the digital pin 13 as an output pin.
    pins.digital[13].set_output();

    loop {
        // Turn on the LED
        pins.digital[13].high();

        // Wait for one second
        rustduino::delay::delay_ms(1000);

        // Turn off the LED
        pins.digital[13].low();

        // Wait for one second
        rustduino::delay::delay_ms(1000);
    }
} //toggling the LED periodically

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
