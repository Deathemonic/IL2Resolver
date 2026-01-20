#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ReflectionProbeMode {
    #[default]
    Baked = 0,
    Realtime = 1,
    Custom = 2,
}
