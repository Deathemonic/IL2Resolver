#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ReflectionProbeRefreshMode {
    #[default]
    OnAwake = 0,
    EveryFrame = 1,
    ViaScripting = 2,
}
