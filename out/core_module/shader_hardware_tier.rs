#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ShaderHardwareTier {
    #[default]
    Tier1 = 0,
    Tier2 = 1,
    Tier3 = 2,
}
