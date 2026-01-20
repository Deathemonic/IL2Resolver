#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ApplicationInstallMode {
    #[default]
    Unknown = 0,
    Store = 1,
    DeveloperBuild = 2,
    Adhoc = 3,
    Enterprise = 4,
    Editor = 5,
}
