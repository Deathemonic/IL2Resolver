#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CollisionFlags {
    #[default]
    None = 0,
    Sides = 1,
    Above = 2,
    Below = 4,
}
