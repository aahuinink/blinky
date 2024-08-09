use esp_idf_svc::{hal::{
	delay::FreeRtos,
	prelude::*},
	sys};
use blinky::rgb::neopixel::{Neopixel, WS2812};
fn main() {
	// It is necessary to call this function once. Otherwise some patches to the runtime
	// implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
	sys::link_patches();

	let peripherals = Peripherals::take().expect("Failed to take peripherals");
	
	let mut led =WS2812::build(peripherals.pins.gpio8, peripherals.rmt.channel0).unwrap();

	loop {
		led.write((125,125,0).into()).unwrap();
		FreeRtos::delay_ms(500);
		led.off().unwrap();
		FreeRtos::delay_ms(500);
	}
}
