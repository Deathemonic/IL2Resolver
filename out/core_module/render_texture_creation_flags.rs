#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RenderTextureCreationFlags {
    #[default]
    MipMap = 1,
    AutoGenerateMips = 2,
    SRGB = 4,
    EyeTexture = 8,
    EnableRandomWrite = 16,
    CreatedFromScript = 32,
    AllowVerticalFlip = 128,
    NoResolvedColorSurface = 256,
    DynamicallyScalable = 1024,
    BindMS = 2048,
}
