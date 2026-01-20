#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DirectorUpdateMode {
    #[default]
    DSPClock = 0,
    GameTime = 1,
    UnscaledGameTime = 2,
    Manual = 3,
}
