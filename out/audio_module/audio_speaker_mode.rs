#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AudioSpeakerMode {
    #[default]
    Raw = 0,
    Mono = 1,
    Stereo = 2,
    Quad = 3,
    Surround = 4,
    Mode5point1 = 5,
    Mode7point1 = 6,
    Prologic = 7,
}
