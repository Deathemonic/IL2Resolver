#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FullScreenMode {
    #[default]
    ExclusiveFullScreen = 0,
    FullScreenWindow = 1,
    MaximizedWindow = 2,
    Windowed = 3,
}
