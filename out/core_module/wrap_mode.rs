#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WrapMode {
    Once = 1,
    Loop = 2,
    PingPong = 4,
    #[default]
    Default = 0,
    ClampForever = 8,
}
