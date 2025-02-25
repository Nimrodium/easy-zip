use colorize::AnsiColor;
use std::{
    path::{Path, PathBuf},
    process::exit,
};
use zip_extensions;
const NAME: &str = "ezip";
fn zip(args: &ParsedArgs) -> Result<(), String> /* i dunno what its gonna Err return */ {
    let src_directory = Path::new(&args.src).to_path_buf();
    let dest_archive = Path::new(&args.dest).to_path_buf();

    if !src_directory.is_dir() {
        return Err(format!("{src_directory:?} is not a directory"));
    }

    // if !zip_extensions::is_zip(&dest_archive) {
    //     return Err(format!("{dest_archive:?} is not a zip file"));
    // }
    zip_extensions::zip_create_from_directory(&dest_archive, &src_directory)
        .map_err(|e| e.to_string())?;
    Ok(())
}

fn unzip(args: &ParsedArgs) -> Result<(), String> /* i dunno what its gonna Err return */ {
    let src_archive = Path::new(&args.src).to_path_buf();
    let dest_directory = Path::new(&args.dest).to_path_buf();
    if !zip_extensions::is_zip(&src_archive) {
        return Err(format!("{src_archive:?} is not a zip file"));
    }
    // if !dest_directory.is_dir() {
    //     return Err(format!("{dest_directory:?} is not a directory"));
    // }
    zip_extensions::zip_extract(&src_archive, &dest_directory).map_err(|e| e.to_string())?;
    Ok(())
}

fn help() -> ! {
    println!(
        "{NAME} [-x / --extract | -c / --compress] [src] <optional>[dest]\n\
        Options:\n\
            \t-c | --compress -- compress src directory to dest archive file\n\
            \t\tif no dest is provided then <src>.zip will be used\n\
            \t-x | --extract -- extract src archive file to dest directory\n\
            \t\tif no dest is provided then if <src> ends in .zip the .zip will be removed.\n\
            \t\tif not then <src>.extracted will be used
        "
    );
    exit(0)
}

fn handle_err(err: &str) -> ! {
    eprintln!("{NAME}: {} {}", "error".red(), err);
    exit(1)
}
#[derive(Debug)]
enum Mode {
    Compress,
    Extract,
}
#[derive(Debug)]
struct ParsedArgs {
    mode: Mode,
    src: PathBuf,
    dest: PathBuf,
    // other args ...
}
impl ParsedArgs {
    fn parse(args: &[String]) -> Result<Self, String> {
        let mut mode: Option<Mode> = None;
        // let src: Option<String> = None;
        // let dest: Option<String> = None;
        let mut files: Vec<String> = vec![];
        let mut skip_0 = true;
        for arg in args {
            if skip_0 {
                skip_0 = false;
                continue;
            }
            match arg.as_str() {
                "-x" | "--extract" => mode = Some(Mode::Extract),
                "-c" | "--compress" => mode = Some(Mode::Compress),
                "-h" | "--help" => help(),
                a if a.starts_with("-") || a.starts_with("--") => {
                    return Err(format!("malformed flag {a}"))
                }
                _ => files.push(arg.to_string()),
            }
        }
        let unwrapped_mode = if let Some(m) = mode {
            m
        } else {
            return Err("missing mode".to_string());
        };

        let unwrapped_src = if let Some(s) = files.get(0) {
            s
        } else {
            return Err("missing source".to_string());
        };
        let unwrapped_dest = if let Some(d) = files.get(1) {
            d.clone()
        } else {
            match unwrapped_mode {
                Mode::Compress => unwrapped_src.clone() + ".zip",
                Mode::Extract => {
                    let ending = ".zip";
                    let dest = if unwrapped_src.ends_with(ending) {
                        let mut d = unwrapped_src.clone();
                        // trunacate length - ending.len()

                        d.truncate(d.len() - ending.len());
                        d
                    } else {
                        let d = unwrapped_src.clone();
                        d + ".extracted"
                    };
                    dest.clone()
                }
            }
        };
        Ok(Self {
            mode: unwrapped_mode,
            src: PathBuf::from(unwrapped_src),
            dest: PathBuf::from(unwrapped_dest),
        })
    }
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    // parse args
    // route either zip or unzip
    // anything after -O flag is passed to zip or unzip
    //
    // exmp:
    // bzip -x archive.zip archive : extracts archive.zip to archive
    // bzip -c archive <optional>archive.zip compresses archive folder to archive.zip
    //      (if out file not provided it defaults to <dirname>.zip)
    //
    let parsed_args = match ParsedArgs::parse(&args) {
        Ok(args) => args,
        Err(err) => handle_err(&err),
    };
    println!("{parsed_args:?}");
    match match parsed_args.mode {
        Mode::Compress => zip(&parsed_args),
        Mode::Extract => unzip(&parsed_args),
    } {
        Ok(()) => (),
        Err(err) => handle_err(&err),
    }
}
