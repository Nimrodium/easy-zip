use crate::formats::ArchiveFormat;

pub struct Zip {}

impl ArchiveFormat for Zip {
    fn compress(
        &self,
        sources: &[&std::path::Path],
        archive: &std::path::Path,
    ) -> Result<(), String> {
        todo!()
    }

    fn extract(&self, archive: &std::path::Path, target: &std::path::Path) -> Result<(), String> {
        todo!()
    }
}
