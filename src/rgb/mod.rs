pub mod rgb {
    pub mod neopixel;

    pub trait Colour {}

    pub struct Red(u8);
    pub struct Green(u8);
    pub struct Blue(u8);

    impl Colour for Red {}
    impl Colour for Green {}
    impl Colour for Blue {}

    pub struct RgbVal(Red, Green, Blue);
}

