# releasor
  <a href="https://crates.io/crates/releasor">
    <img src="https://img.shields.io/crates/v/releasor?style=flat&labelColor=1C2C2E&color=C96329&logo=Rust&logoColor=white" /></a>
    
  <a href="https://github.com/joaolfp/releasor/actions/workflows/CI.yml">
    <img src="https://img.shields.io/github/actions/workflow/status/joaolfp/releasor/CI.yml?style=flat&labelColor=1C2C2E&label=CI&color=BEC5C9&logo=GitHub%20Actions&logoColor=BEC5C9" /></a>
    
  <a href="https://github.com/joaolfp/releasor/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/license-MIT-BEC5C9?style=flat&labelColor=1C2C2E&logoColor=BEC5C9" /></a>
</p>

A command-line tool to automate release tasks for Rust 🦀 projects and libraries. releasor streamlines building, packaging into `.tar.gz` archives, and verifying binaries, providing a fast and consistent release workflow.

##  Features
- Automated cargo build --release execution
- Creation of compressed .tar.gz archives with your chosen name
- Generation and printout of SHA-256 checksums for integrity verification

## Installing

#### Cargo 🦀
Installing from [crates.io](https://crates.io/) (requires Rust/Cargo):

```shell
cargo install releasor
```

## Usage

```sh
releasor -f package_name
```

## Contributing

To contribute, just fork this project and then open a pull request, feel free to contribute, bring ideas and raise any problem in the issue tab.

## License

releasor is released under the MIT license. See [LICENSE](https://github.com/joaolfp/releasor/blob/main/LICENSE) for details.
