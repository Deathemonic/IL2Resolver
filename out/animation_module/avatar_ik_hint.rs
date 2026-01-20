#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AvatarIKHint {
    #[default]
    LeftKnee = 0,
    RightKnee = 1,
    LeftElbow = 2,
    RightElbow = 3,
}
