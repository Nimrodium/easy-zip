use std::fs::File;

use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use tar::Archive;

use crate::formats::{ArchiveFormat, Options};

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct TarGz;

impl ArchiveFormat for TarGz {
    fn compress(
        &self,
        sources: &[std::path::PathBuf],
        archive: &std::path::Path,
        options: Options,
    ) -> Result<(), String> {
        let file =
            File::create(archive).map_err(|e| format!("could not create zip archive {e}"))?;
        let compression = if let Some(c) = options.compression_level {
            Compression::new(c as u32)
        } else {
            Compression::default()
        };
        let enc = GzEncoder::new(file, compression);
        let mut tar = tar::Builder::new(enc);
        for source in sources {
            if source.is_dir() {
                tar.append_dir_all("", source)
                    .map_err(|e| format!("failed to add source to archive {e}"))?;
            } else {
                tar.append_path(source)
                    .map_err(|e| format!("could not archive source file {source:?}: {e}"))?;
            }
        }
        tar.finish()
            .map_err(|e| format!("failed to complete write to {archive:?} tar.gz archive: {e}"))?;

        Ok(())
    }

    fn extract(
        &self,
        archive_file: &std::path::Path,
        target: &std::path::Path,
    ) -> Result<(), String> {
        let file = File::open(archive_file).map_err(|e| format!("could not open archive {e}"))?;
        let mut archive = Archive::new(GzDecoder::new(file));
        archive.unpack(target).map_err(|e| {
            format!("failed to unpack tar.gz archive {archive_file:?} -> {target:?}: {e}")
        })?;
        Ok(())
    }
}
