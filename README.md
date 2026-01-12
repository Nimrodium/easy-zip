<!--# easy-zip
a more easy way to quickly compress and decompress zip files. 

i made this because the zip and unzip commands (mostly unzip) are for some reason SO confusing for me !!! 
i still have no idea how to use them and i have to look it up every single time. so i finally snapped and cooked up this quick rust program to zip stuff in a mentally sane syntax, its less feature rich but cmon be honest youre only ever unzipping and zipping, if youre using zip/unzip for anything else youre copying it from a command on ask ubuntu.-->

# Sticky
Basic compressed archive tool with a sane syntax for easily working with compressed archives.


# Flags
* `-e` `--extract` 
> extract archive to SOURCE/
* `-c` `--compress`
> compress sources to archive
* `-t` `--target`
> archive / destination directory name
* `-h` `--help`
> show help message
* `-f` `--format`
> archival format (zip tar.gz zstd)
* `-l` `--level`
> compression level
* `-v` `--verbose`
> enable verbose output



# Usage
* `sticky archive.zip`
> extracts archive.zip to archive/
* `sticky archive.zip -t extracted`
> extracts archive.zip to extracted/
* `sticky -e archive.zip -t archive/ -f zip`
> same as `sticky archive.zip`
* `sticky file1.txt file2.txt file3.md`
> compresses file1.txt file2.txt file3.md to `archive.zip`
* `sticky file1.txt file2.txt file3.md -t files.tar.gz`
> compresses file1.txt file2.txt file3.md to files.tar.gz using tar.gz compression

sticky can infer what you want to do via the file extensions of `--target` and source files, 
however it will fail if sources contains both compressed archives and uncompressed files. in this case, the `-e`/`-c` flags are required to disambiguate

# Installation
## Cargo
```bash
# git clone  && cd sticky
cargo install --git https://github.com/nimrodium/sticky.git
```
## Nix
```nix
# in your flake.nix inputs
sticky = {
      url = "github:nimrodium/sticky";
      inputs.nixpkgs.follows = "nixpkgs";
    };
```

```nix
# in your systemPackages, make sure you have inputs passed to your configuration.nix or explicitly pass sticky
with pkgs; [ inputs.sticky.packages.${pkgs.stdenv.hostPlatform.system}.default ];
```
# why
i can never remember all the random flags required to extract all the different archive formats, so this is a rust cli tool for extracting archives agnostically
sticky because, like, you stick a bunch of files together haphazardly
