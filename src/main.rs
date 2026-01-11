use clap::Parser;
use colorize::AnsiColor;
use std::{
    path::{Path, PathBuf},
    str::FromStr,
};
mod formats;
mod utils;
use crate::formats::Format;
const NAME: &str = "ezip";
macro_rules! error {
    () => {
        eprintln!()
    };
    ($($arg:tt)*) => {{
        eprintln!("{}: {} {}", crate::NAME, "error".red(),format!($($arg)*));
    }};
}
#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    /// extract source to target dir
    extract: bool,
    #[arg(short, long, default_value_t = false)]
    /// compress source to target dir
    compress: bool,
    /// source directory
    sources: Vec<String>,
    #[arg(short, long)]
    /// target directory
    target: String,
    #[arg(short, long, default_value_t = Format::Zip)]
    format: Format,
}

#[derive(Debug, Clone)]
enum Mode {
    Compress,
    Extract,
}

fn infer_format(file: &Path) -> Option<Format> {
    utils::extract_file_extension(file)
        .map(|ext| Format::from_str(&ext).ok())
        .unwrap_or(None)
}
// is compress if sources are directories or non-zip files
// is extract if sources are valid formats which are decided by ext
// fn infer_mode(sources: &[&Path]) -> Option<(Mode, Format)> {
//     // format = get_format_from_target(file);
//     let extensions = sources
//         .iter()
//         .map(|f| f.extension().and_then(|ext| ext.to_str()));
//     let is_compress = sources.iter().zip(extensions).map(
//         |(f,ext)| f.is_dir() ||
//     );
// }

fn main() -> Result<(), String> {
    println!(
        "{:?}",
        utils::extract_file_extension(&PathBuf::from("spoink/.tar.zstd.louis"))
    );
    // let args = Args::parse();

    Ok(())
}
