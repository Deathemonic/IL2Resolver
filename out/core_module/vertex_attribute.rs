#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VertexAttribute {
    #[default]
    Position = 0,
    Normal = 1,
    Tangent = 2,
    Color = 3,
    TexCoord0 = 4,
    TexCoord1 = 5,
    TexCoord2 = 6,
    TexCoord3 = 7,
    TexCoord4 = 8,
    TexCoord5 = 9,
    TexCoord6 = 10,
    TexCoord7 = 11,
    BlendWeight = 12,
    BlendIndices = 13,
}
