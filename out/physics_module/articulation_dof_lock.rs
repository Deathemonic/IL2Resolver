#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ArticulationDofLock {
    #[default]
    LockedMotion = 0,
    LimitedMotion = 1,
    FreeMotion = 2,
}
