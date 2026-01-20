#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AdditionalCanvasShaderChannels {
    #[default]
    None = 0,
    TexCoord1 = 1,
    TexCoord2 = 2,
    TexCoord3 = 4,
    Normal = 8,
    Tangent = 16,
}
