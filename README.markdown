# COUTILS :gear:

![GitHub CI](https://github.com/alyxshang/coutils/actions/workflows/rust.yml/badge.svg)

***A set of useful functions for Rust. :gear:***

## ABOUT :books:

This repository contains a Rust crate containing APIs for performing basic tasks in Rust. Some of these tasks are: interacting with the filesystem, manipulating strings, manipulating numbers, and many more.

## INSTALLATION :inbox_tray:

To use ***Coutils*** in your Rust project, add this line to your project's `Cargo.toml`'s `[dependencies]` section:

```TOML
coutils = { git = "https://github.com/alyxshang/coutils", tag = "v.0.2.0" }
```

Optionally, you can decide which features you would like to enable. ***Coutils*** offers the following features:

- `filesystem`: A feature to work with files and directories.
- `clone`: A feature to clone Git repositories.
- `time`: A feature to get information on the current time.
- `loading`: Download a file from the internet.
- `compression`: Create and extract tarballs.

To enable one or all of the available features, add this line to your Rust project's `Cargo.toml`:

```TOML
coutils = { git = "https://github.com/alyxshang/coutils", tag = "v.0.2.0", features = ["feature_name"] }
```

The placeholder `feature_name` represents any of the features this crate contains. The feature(s) enabled depend on the requirements of your project.

## USAGE :hammer:

To view this crate's API please clone this repository and run the command `cargo doc --open` from the repository's root.

## CHANGELOG :black_nib:

### Version 0.1.0

- Initial release.
- Upload to GitHub.

### Version 0.2.0

- Added more documentation.
- Added more features.
- Added a function for downloading files.
- Added a function for compressing a directory into a tarball.
- Added a function for extracting a directory from a tarball.
- Made many file functions more friendly for cross-platform usage.

## NOTE :scroll:

- *Coutils :gear:* by *Alyx Shang :black_heart:*.
- Licensed under the [FSL v1](https://github.com/alyxshang/fair-software-license).
