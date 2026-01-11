// fn unzip(args: &ParsedArgs) -> Result<(), String> /* i dunno what its gonna Err return */ {
//     let src_archive = Path::new(&args.src).to_path_buf();
//     let dest_directory = Path::new(&args.dest).to_path_buf();
//     if !zip_extensions::is_zip(&src_archive) {
//         return Err(format!("{src_archive:?} is not a zip file"));
//     }
//     // if !dest_directory.is_dir() {
//     //     return Err(format!("{dest_directory:?} is not a directory"));
//     // }
//     zip_extensions::zip_extract(&src_archive, &dest_directory).map_err(|e| e.to_string())?;
//     Ok(())
// }
