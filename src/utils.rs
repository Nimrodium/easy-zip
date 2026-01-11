use std::path::Path;

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
macro_rules! error {
    () => {
        eprintln!()
    };
    ($($arg:tt)*) => {{
        eprintln!("{}: {} {}", crate::NAME, "error".red(),format!($($arg)*));
    }};
}

macro_rules! verbose {
    () => {
        if crate::VERBOSE {
            println!()
        }
    };
    ($($arg:tt)*) => {{
        if crate::VERBOSE {
            println!("{}: {} {}", crate::NAME, "error".red(),format!($($arg)*));
        }
    }};
}
