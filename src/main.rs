use clap::{CommandFactory, Parser};
use colorize::AnsiColor;
use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

mod formats;
#[macro_use]
mod utils;
use crate::formats::{ArchiveFormat, Format, Options};
const NAME: &str = "sticky";
static mut VERBOSE: bool = false;

#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    /// extract source to target dir
    extract: bool,
    #[arg(short, long, default_value_t = false)]
    /// compress source to target dir
    compress: bool,
    #[arg(required = true,num_args = 1..)]
    /// source files
    sources: Vec<String>,
    #[arg(short, long)]
    /// target directory/archive
    target: Option<String>,
    #[arg(short, long)]
    format: Option<Format>,
    #[arg(short, long)]
    /// compression level
    level: Option<i64>,
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

#[derive(Debug, Clone)]
enum Mode {
    Compress,
    Extract,
}

fn infer_format(archive: &Path) -> Option<Format> {
    utils::extract_file_extension(archive)
        .map(|ext| Format::from_str(&ext).ok())
        .unwrap_or(None)
}
fn infer(sources: &[PathBuf], target: &Option<PathBuf>) -> (Option<Mode>, Option<Format>) {
    let files = sources.iter().filter(|p| p.is_file());
    // if any directories are present, must be compress. if any valid formats are present in the files, then ambigious.
    let dirs = sources.iter().map(|f| f.is_dir()).filter(|d| *d).count();

    // if any valid formats are present, must be extract. else compress. if any non-archive files present, then ambigious
    // let file_formats = files.map(|(ext, _)| Format::from_str(&ext).ok());
    let file_formats =
        files.map(|f| utils::extract_file_extension(f).and_then(|ext| Format::from_str(&ext).ok()));
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

fn main() -> Result<(), String> {
    let args = Args::parse();
    let options = Options::new(args.level);
    let sources = args
        .sources
        .iter()
        .map(|s| PathBuf::from(s))
        .collect::<Vec<PathBuf>>();
    unsafe { VERBOSE = args.verbose };
    let target = args.target.map(|t| PathBuf::from(t));
    let (mode, format) = {
        let (op_mode, op_format) = infer(&sources, &target);
        println!("{op_mode:?} {op_format:?}");
        let mode = if args.compress && args.extract {
            return Err("cannot use compress and extract at the same time!".to_string());
        } else if args.compress {
            Mode::Compress
        } else if args.extract {
            Mode::Extract
        } else {
            if let Some(m) = op_mode {
                m
            } else {
                return Err(
                    "cannot infer extract/compress, please use --extract, --compress flags"
                        .to_string(),
                );
            }
        };
        let format = if let Some(f) = args.format {
            f
        } else {
            if let Some(f) = op_format {
                f // vec is stupid actually
            } else {
                Format::default()
                // return Err("cannot infer archive format, please use -f flag".to_string());
            }
        };
        (mode, format)
    };
    match mode {
        Mode::Compress => {
            let target = if let Some(t) = target {
                t
            } else {
                sources[0].with_added_extension(format.get_extension())
            };
            format.compress(&sources, &target, options)?;
        }
        Mode::Extract => {
            let archive = sources[0].clone();
            let target = if let Some(t) = target {
                t
            } else {
                PathBuf::from(
                    archive
                        .to_string_lossy()
                        .strip_suffix(&(".".to_string() + &format.get_extension().as_str()))
                        .unwrap_or(&archive.to_string_lossy())
                        .to_string(),
                )
            };
            format.extract(&archive, &target)?;
        }
    }
    Ok(())
}
