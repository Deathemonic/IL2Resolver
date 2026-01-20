#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AssetBundleLoadResult {
    #[default]
    Success = 0,
    Cancelled = 1,
    NotMatchingCrc = 2,
    FailedCache = 3,
    NotValidAssetBundle = 4,
    NoSerializedData = 5,
    NotCompatible = 6,
    AlreadyLoaded = 7,
    FailedRead = 8,
    FailedDecompression = 9,
    FailedWrite = 10,
    FailedDeleteRecompressionTarget = 11,
    RecompressionTargetIsLoaded = 12,
    RecompressionTargetExistsButNotArchive = 13,
}
