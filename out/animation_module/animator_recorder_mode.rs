#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AnimatorRecorderMode {
    #[default]
    Offline = 0,
    Playback = 1,
    Record = 2,
}
