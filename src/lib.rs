pub mod rgb{
    
    struct Red(u8);
    struct Green(u8);
    struct Blue(u8);

    pub struct RgbVal(Red, Green, Blue);

    /// destructures into a tuple of u8's and consumes self.
    impl RgbVal {

        pub fn new(r: u8, g: u8, b: u8) -> Self {
            Self(Red(r), Green(g), Blue(b))
        }

        pub fn unwrap(self) -> (u8, u8, u8) {
            let (Red(red), Green(green), Blue(blue)) = (self.0, self.1, self.2);
            (red, green, blue)
        }

        pub fn set(mut self, r: u8, g: u8, b: u8) -> Self {
            self.0 = Red(r);
            self.1 = Green(g);
            self.2 = Blue(b);
            self
        }
    }

    impl Default for RgbVal {
        // returns a black/off (0,0,0) rgb value
        fn default() -> Self {
            Self(Red(0), Green(0), Blue(0))
        }
    }

    impl From<RgbVal> for u32 {
        fn from(rgb: RgbVal) -> Self {
            let (r, g, b) = rgb.unwrap();
            (r as u32) << 16 | (g as u32) << 8 | (b as u32)
        }
    }

    impl From<(u8, u8, u8)> for RgbVal {
        fn from(rgb: (u8, u8, u8)) -> RgbVal {
            RgbVal::new(rgb.0, rgb.1, rgb.2)
        }
    }

    pub mod neopixel{

        use super::RgbVal;
        use std::time::Duration;
        use esp_idf_svc::{
            hal::{
                gpio::OutputPin, 
                peripheral::Peripheral, rmt::*}, 
            sys::*,};

        

        pub trait Neopixel {
            fn write(&mut self, val: RgbVal) -> Result<(), EspError>;
        }

        pub struct WS2812<'d> {
            led_driver: TxRmtDriver<'d>,
        }

        impl<'d> WS2812<'d> {

            pub fn build<P, C>(pin: P, rmt_channel: impl Peripheral<P = C> + 'd) -> Result<Self, EspError> 
                where
                    P: OutputPin + 'd,
                    C: RmtChannel 
            {
                // create rmt config
                let config: TxRmtConfig = TxRmtConfig::new().clock_divider(1);
                let tx = TxRmtDriver::new(rmt_channel, pin, &config)?;
                Ok(Self { led_driver: tx })
            }

            pub fn on(&mut self) -> Result<(), EspError> {
                self.write((255,25,255).into())
            }

            pub fn off(&mut self) -> Result<(), EspError> {
                self.write((0,0,0).into())
            }
        }


        impl<'d> Neopixel for WS2812<'d> {

            fn write(&mut self, val: RgbVal) -> Result<(), EspError> {
                // create our pulse codes
                    // get clock
                let ticks_hz = self.led_driver.counter_clock()?;
                    // make codes from duration
                let (t0h, t1h, t0l, t1l) = (
                    Pulse::new_with_duration(ticks_hz, PinState::High, &Duration::from_nanos(400))?,
                    Pulse::new_with_duration(ticks_hz, PinState::High,  &Duration::from_nanos(800))?,
                    Pulse::new_with_duration(ticks_hz, PinState::Low, &Duration::from_nanos(850))?,
                    Pulse::new_with_duration(ticks_hz, PinState::Low, &Duration::from_nanos(450))?,
                );
                // create a pulse train signal with our data
                    // convert to u32
                let mut colour: u32 = val.into();
                let mut signal: FixedLengthSignal<24> = FixedLengthSignal::<24>::new();
                    // create pulse pairs
                for i in 0..24 {
                    // use bit mask to determine if msb is 1 or 0
                    if (colour & 0x800000) > 0 {
                        signal.set(i, &(t1h, t1l))?;
                    } else {
                        signal.set(i, &(t0h, t0l))?;
                    }
                    colour <<= 1;
                }
                // transmit the data
                self.led_driver.start(signal)?;
                Ok(())
            }
        }
    }
}