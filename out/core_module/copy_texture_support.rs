#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CopyTextureSupport {
    #[default]
    None = 0,
    Basic = 1,
    Copy3D = 2,
    DifferentTypes = 4,
    TextureToRT = 8,
    RTToTexture = 16,
}
