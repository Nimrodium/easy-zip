use std::{path::Path, str::FromStr};

mod targz;
mod zip;
mod zstd;

pub trait ArchiveFormat {
    fn compress(&self, sources: &[&Path], archive: &Path) -> Result<(), String>;
    fn extract(&self, archive: &Path, target: &Path) -> Result<(), String>;
}

#[derive(Debug, Clone)]
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
    fn compress(&self, sources: &[&Path], archive: &Path) -> Result<(), String> {
        todo!()
    }

    fn extract(&self, archive: &Path, target: &Path) -> Result<(), String> {
        todo!()
    }
}
