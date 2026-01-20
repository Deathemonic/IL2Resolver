#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PhysicMaterialCombine {
    #[default]
    Average = 0,
    Minimum = 2,
    Multiply = 1,
    Maximum = 3,
}
