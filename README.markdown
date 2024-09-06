# COUTILS :gear:

![GitHub CI](https://github.com/alyxshang/coutils/actions/workflows/rust.yml/badge.svg)

***A set of useful functions for Rust. :gear:***

## ABOUT :books:

This repository contains a Rust crate containing APIs for performing basic tasks in Rust. Some of these tasks are: interacting with the filesystem, manipulating strings, manipulating numbers, and many more.

## INSTALLATION :inbox_tray:

To use ***Coutils*** in your Rust project, add this line to your project's `Cargo.toml`'s `[dependencies]` section:

```TOML
coutils = { git = "https://github.com/alyxshang/coutils", version = "0.1.0" }
```

Optionally, you can decide which features you would like to enable. ***Coutils*** offers the following features:

- `filesystem`: A feature to work with files and directories.
- `networking`: A feature to clone Git repositories.
- `time`: A feature to get current time information.

To enable one or all of the available features, add this line to your Rust project's `Cargo.toml`:

```TOML
coutils = { git = "https://github.com/alyxshang/coutils", version = "0.1.0", features = ["feature_name"] }
```

The placeholder `feature_name` represents any of the features this crate contains. The feature(s) you enable depend on the requirements of your project.

## USAGE :hammer:

To view this crate's API, please visit the [documentation](https://alyxshang.boo/coutils/).

## CHANGELOG :black_nib:

### Version 0.1.0

- Initial release.
- Upload to GitHub.

## NOTE :scroll:

- *Coutils :gear:* by *Alyx Shang :black_heart:*.
- Licensed under the [FSL v1](https://github.com/alyxshang/fair-software-license).
