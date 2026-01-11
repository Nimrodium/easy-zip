use clap::Parser;
use colorize::AnsiColor;
use std::{
    path::{Path, PathBuf},
    process::exit,
    str::FromStr,
};
use zip_extensions;
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
enum Format {
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
impl Format {}
#[derive(Debug, Clone)]
enum Mode {
    Compress,
    Extract,
}

fn infer_format(file: &Path) -> Option<Format> {
    file.extension()
        .and_then(|ext| {
            ext.to_str()
                .and_then(|ext| Some(Format::from_str(ext).ok()))
        })
        .unwrap_or(None)
}
// is compress if sources are directories or non-zip files
// is extract if sources are valid formats which are decided by ext
fn infer_mode(sources: &[&Path]) -> Option<(Mode, Format)> {
    // format = get_format_from_target(file);
    let extensions = sources
        .iter()
        .map(|f| f.extension().and_then(|ext| ext.to_str()));
    let is_compress = sources.iter().zip(extensions).map(
        |(f,ext)| f.is_dir() ||
    );
}

fn main() -> Result<(), String> {
    // let args: Vec<String> = std::env::args().collect();
    // parse args
    // route either zip or unzip
    // anything after -O flag is passed to zip or unzip
    //
    //
    // exmp:
    // bzip -x archive.zip archive : extracts archive.zip to archive
    // bzip -c archive <optional>archive.zip compresses archive folder to archive.zip
    //      (if out file not provided it defaults to <dirname>.zip)
    //

    let args = Args::parse();
    // if !(args.extract || args.compress) {
    //     error!("no mode provided");
    // }
    // let mode: Mode = match (args.extract, args.compress) {
    //     // kinda evil and gross
    //     (true, false) => Mode::Extract,
    //     (false, true) => Mode::Compress,
    //     (true, true) => {
    //         error!("cannot use extract and compress at the same time!");
    //         exit(1)
    //     }
    //     (false, false) => {
    //         error!("no mode provided!");
    //         exit(1)
    //     }
    // };

    Ok(())

    // let parsed_args = match ParsedArgs::parse(&args) {
    //     Ok(args) => args,
    //     Err(err) => handle_err(&err),
    // };
    // let args = println!("{parsed_args:?}");
    // match match parsed_args.mode {
    //     Mode::Compress => zip(&parsed_args),
    //     Mode::Extract => unzip(&parsed_args),
    // } {
    //     Ok(()) => (),
    //     Err(err) => handle_err(&err),
    // }
}
