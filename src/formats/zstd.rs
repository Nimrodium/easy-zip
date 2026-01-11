use crate::formats::{ArchiveFormat, Options};
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct Zstd;
impl ArchiveFormat for Zstd {
    fn compress(
        &self,
        sources: &[std::path::PathBuf],
        archive: &std::path::Path,
        options: Options,
    ) -> Result<(), String> {
        todo!()
    }

    fn extract(&self, archive: &std::path::Path, target: &std::path::Path) -> Result<(), String> {
        todo!()
    }
}
