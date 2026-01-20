#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AnimationPlayMode {
    #[default]
    Stop = 0,
    Queue = 1,
    Mix = 2,
}
