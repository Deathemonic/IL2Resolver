#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AudioDataLoadState {
    #[default]
    Unloaded = 0,
    Loading = 1,
    Loaded = 2,
    Failed = 3,
}
