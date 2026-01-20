#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RenderBufferStoreAction {
    #[default]
    Store = 0,
    Resolve = 1,
    StoreAndResolve = 2,
    DontCare = 3,
}
