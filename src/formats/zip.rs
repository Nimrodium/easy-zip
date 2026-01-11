use std::fs::File;

use zip::{write::FileOptions, ZipArchive, ZipWriter};

use crate::formats::{ArchiveFormat, Options};

pub struct Zip;

impl ArchiveFormat for Zip {
    fn compress(
        &self,
        sources: &[std::path::PathBuf],
        archive: &std::path::Path,
        options: Options,
    ) -> Result<(), String> {
        let file =
            File::create(archive).map_err(|e| format!("could not create zip archive {e}"))?;
        let mut archive_file = ZipWriter::new(file);
        let zip_options = FileOptions::DEFAULT.compression_level(options.compression_level);
        for source in sources {
            archive_file
                .start_file_from_path(source, zip_options)
                .map_err(|e| {
                    format!("could not write {source:?} to {archive:?} zip archive: {e}")
                })?;
        }
        archive_file
            .finish()
            .map_err(|e| format!("failed to complete write to {archive:?} zip archive: {e}"))?;
        Ok(())
    }

    fn extract(
        &self,
        archive_file: &std::path::Path,
        target: &std::path::Path,
    ) -> Result<(), String> {
        let file = File::open(archive_file).map_err(|e| format!("could not open archive {e}"))?;
        let mut archive =
            ZipArchive::new(file).map_err(|e| format!("could not read archive {e}"))?;
        archive
            .extract(target)
            .map_err(|e| format!("failed to extract archive {e}"))?;
        Ok(())
    }
}
impl Zip {
    pub fn default() -> Self {
        Self
    }
}
