use crate::formats::Format;

use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

pub fn extract_file_extension(path: &Path) -> Option<String> {
    // does not use p.extension as i want to catch .tar.gz
    let name = path.file_name().and_then(|p| Some(p.to_string_lossy()));
    println!("{name:?}");
    let ext: Option<String> = name
        .as_ref()
        .and_then(|n| {
            n.chars()
                .position(|e| e == '.')
                .and_then(|i| if i == 0 { None } else { Some(i) }) // stop for hidden files
                .and_then(|i| Some(n.len() - i - 1))
        })
        .inspect(|i| println!("{i}"))
        .and_then(|i| name.and_then(|n| Some(n.chars().rev().take(i).collect())));

    ext.map(|ext| ext.chars().rev().collect::<String>())
}
#[derive(Debug, Clone)]
pub enum Mode {
    Compress,
    Extract,
}
pub fn infer(sources: &[PathBuf], target: &Option<PathBuf>) -> (Option<Mode>, Option<Format>) {
    let files = sources.iter().filter(|p| p.is_file());
    // if any directories are present, must be compress. if any valid formats are present in the files, then ambigious.
    let dirs = sources.iter().map(|f| f.is_dir()).filter(|d| *d).count();

    // if any valid formats are present, must be extract. else compress. if any non-archive files present, then ambigious
    // let file_formats = files.map(|(ext, _)| Format::from_str(&ext).ok());
    let file_formats =
        files.map(|f| extract_file_extension(f).and_then(|ext| Format::from_str(&ext).ok()));
    let non_archive = file_formats.clone().filter(|f| f.is_none()).count();
    let formats = file_formats
        .filter(|f| f.is_some())
        .map(|f| f.unwrap())
        .collect::<Vec<Format>>();
    let format = if formats.iter().all(|f| Some(f) == formats.get(0)) {
        formats.get(0).cloned()
    } else {
        None
    };
    let archive_count = formats.len();
    let target_format = target
        .as_ref()
        .map(|t| Format::format_from_path(t))
        .unwrap_or(None);
    // if only directories are present, then must be compress
    // if no archives are present, must be compress
    // if only archives are present, must be extract
    // if archives exist with any other files/dirs, ambigious
    let ambigious = (None, None);
    let compress = Some(Mode::Compress);
    let extract = Some(Mode::Extract);

    match (archive_count != 0, dirs != 0, non_archive != 0) {
        (true, true, true) => ambigious,
        (true, true, false) => ambigious,
        (true, false, true) => ambigious,
        (false, false, false) => ambigious,
        (true, false, false) => (extract, format),
        _ => (compress, target_format),
    }
}

#[macro_export]
macro_rules! error {
    () => {
        eprintln!()
    };
    ($($arg:tt)*) => {{
        eprintln!("{}: {} {}", crate::NAME, "error".red(),format!($($arg)*));
    }};
}

#[macro_export]
macro_rules! verbose {
    () => {
        unsafe {
            if crate::VERBOSE {
                println!()
            }
        }
    };
    ($($arg:tt)*) => {{
       unsafe { if crate::VERBOSE {
            println!("{}: {}", crate::NAME,format!($($arg)*).yellow());
       }}
    }};
}

#[macro_export]
macro_rules! success {
    () => {
        {println!()}
    };
    ($($arg:tt)*) => {{
        println!("{}: {}", crate::NAME,format!($($arg)*).green());
    }};
}
