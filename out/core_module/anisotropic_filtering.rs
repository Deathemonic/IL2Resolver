#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AnisotropicFiltering {
    #[default]
    Disable = 0,
    Enable = 1,
    ForceEnable = 2,
}
