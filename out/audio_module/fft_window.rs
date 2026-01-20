#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FFTWindow {
    #[default]
    Rectangular = 0,
    Triangle = 1,
    Hamming = 2,
    Hanning = 3,
    Blackman = 4,
    BlackmanHarris = 5,
}
