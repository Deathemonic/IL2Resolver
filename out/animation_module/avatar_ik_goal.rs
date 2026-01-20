#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AvatarIKGoal {
    #[default]
    LeftFoot = 0,
    RightFoot = 1,
    LeftHand = 2,
    RightHand = 3,
}
