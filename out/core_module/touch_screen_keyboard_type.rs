#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TouchScreenKeyboardType {
    #[default]
    Default = 0,
    ASCIICapable = 1,
    NumbersAndPunctuation = 2,
    URL = 3,
    NumberPad = 4,
    PhonePad = 5,
    NamePhonePad = 6,
    EmailAddress = 7,
    NintendoNetworkAccount = 8,
    Social = 9,
    Search = 10,
    DecimalPad = 11,
    OneTimeCode = 12,
}
