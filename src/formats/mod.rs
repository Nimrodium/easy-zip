use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use crate::{formats::zip::Zip, utils};

mod targz;
mod zip;
mod zstd;

pub struct Options {
    compression_level: Option<i64>,
}
impl Options {
    pub fn new(compression_level: Option<i64>) -> Self {
        Self { compression_level }
    }
}
// impl Options {
//     fn new(co) -> Self {

//     }
// }
pub trait ArchiveFormat {
    fn compress(&self, sources: &[PathBuf], archive: &Path, options: Options)
        -> Result<(), String>;
    fn extract(&self, archive: &Path, target: &Path) -> Result<(), String>;
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum Format {
    Zip,
    TarGz,
    Zstd,
}
impl ToString for Format {
    fn to_string(&self) -> String {
        match self {
            Self::Zip => "zip",
            Self::TarGz => "tar.gz",
            Self::Zstd => "zstd",
        }
        .to_string()
    }
}
impl FromStr for Format {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "zip" => Ok(Self::Zip),
            "tar.gz" | "targz" => Ok(Self::TarGz),
            "zstd" => Ok(Self::Zstd),
            _ => Err("not a valid archive format ( zip tar.gz zstd )".to_string()),
        }
    }
}
impl ArchiveFormat for Format {
    fn compress(
        &self,
        sources: &[PathBuf],
        archive: &Path,
        options: Options,
    ) -> Result<(), String> {
        match self {
            Format::Zip => Zip::default().compress(sources, archive, options),
            Format::TarGz => todo!(),
            Format::Zstd => todo!(),
        }
    }

    fn extract(&self, archive: &Path, target: &Path) -> Result<(), String> {
        match self {
            Format::Zip => Zip::default().extract(archive, target),
            Format::TarGz => todo!(),
            Format::Zstd => todo!(),
        }
    }
}
impl Format {
    pub fn is_format_ext(ext: &str) -> bool {
        Self::from_str(ext).is_ok()
    }
    pub fn get_extension(&self) -> String {
        self.to_string()
    }
}
