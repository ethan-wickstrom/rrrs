[//]: # (Image in `.assets/logos/logo.webp`)

<h1 align="center">
  <img src=".assets/logos/logo.webp" alt="RRRS Logo" width="300px" height="300px" />
  <br>
</h1>

<div align="center">
  <a href="https://crates.io/crates/rrrs">
    <img src="https://img.shields.io/crates/v/rrrs.svg" alt="rrrs.io Latest Release"/>
  </a>
</div>

RRRS: Rust(ic) Rapid Random Sampler
===================================

Welcome to RRRS, a rapid, hyper-optimized CSV random sampling tool designed with performance and efficiency at its core. Crafted meticulously in Rust, RRRS offers an unparalleled solution for extracting random data samples from CSV files swiftly and effortlessly.

🤨 Why RRRS
-----------

Born out of a frustrating, repetitive process of sampling from unwieldy or enormous CSV files during my time at Washington University in St. Louis, **RRRS (Rust(ic) Rapid Random Sampler)** represents more than just a tool; it's a perhaps slightly redundant, but fun mission to over-optimize and speed up the all-too-familiar frustration of data sampling. As a student navigating the complex waters of data-heavy courses, I found myself constantly bogged down by the inefficiency of existing methods of importing massive datasets into spreadsheet software, waiting for them to load, and then struggling with plugins or scripting to extract the samples I needed. It was clear: there had to be a better way. So, instead of doing my homework, I work on this:

Enter **RRRS**. Developed with the speed and efficiency of Rust, RRRS is my answer to those frustrating hours. It's designed to make random sampling from large CSV files not just faster, but a seamless part of your workflow. This tool is for anyone who's ever felt this nuisance, turning what was once a bottleneck into a smooth, efficient process. With RRRS, I'm excited to share a solution that helped me and is now here to support data enthusiasts and professionals alike in their analytical endeavors.

🚀 Features
-----------

*   **Rapid Random Sampling**: Quickly extract random samples from large CSV files.
*   **Hyper-Optimized Performance**: Leveraging Rust's powerful system-level capabilities for maximum speed.
*   **User-Friendly**: Simple command-line interface to specify input and output with ease.
*   **Flexibility**: Customizable random sampling according to your data analysis needs.
*   **Cross-Platform Compatibility**: Runs seamlessly on any platform supporting Rust.

🛠 Usage
--------

To get started with RRRS, follow these simple steps:

`rrrs -i <input_file_path> -o <output_file_path>`

Upon execution, RRRS will prompt you to enter the desired number of rows to be randomly sampled from your CSV file. The output will be a new CSV file, named with the original file title plus a suffix indicating the number of sampled rows (e.g., `slogan_data-100`). This file will be saved either in the execution path or a specified output directory.

📂 Directory Structure
----------------------

Understand the organization of RRRS with the following directory structure:

```bash
rrrs/
├── Cargo.toml              # Project manifest
├── src/                    # Source files
│   ├── main.rs             # Entry point
│   ├── library.rs          # Library code
│   ├── args.rs             # Argument parsing
│   └── library/            # Library code
│       ├── sampler_ops/        # Sampling operations
│       │   ├── sampler_ops.rs      # Sampling logic
│       └── csv_ops/            # CSV operations
│           ├── csv_loader.rs   # CSV loading functionality
│           └── csv_writer.rs   # CSV writing functionality
└── tests/                  # Automated tests
    ├── args_tests.rs       # Tests for argument parsing
    ├── csv_loader_tests.rs # Tests for CSV loading
    ├── sampler_tests.rs    # Tests for sampling logic
    └── csv_writer_tests.rs # Tests for CSV writing
```

📚 Getting Started
------------------

### MacOS and Linux

To use RRRS, you need to have Rust installed on your machine. If you don't have Rust installed, install it using the following command: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`. *For more information, refer to the official Rust installation guide [here](https://www.rust-lang.org/tools/install).*

Once Rust is installed, you can install RRRS using the following command: `cargo install rrrs`.

### Windows

**Note**: RRRS is not yet supported on Windows. However, you can still use it by installing the Windows Subsystem for Linux.

### Building from Source

To build RRRS from source, you can clone the repository and build it using the following commands (*Note that this is primarily for development purposes*):

```bash
git clone --branch development git@github.com:ethan-wickstrom/rrrs.git
cd rrrs
cargo build --release
cp target/release/rrrs /usr/local/bin
```

🤝 Contributing
---------------

Contributions to RRRS are warmly welcomed. Feel free to open an issue or submit a pull request, whether it's bug reports, feature requests, or code contributions. Please refer to the contributing guidelines for more details.

**Please note to clone from the `development` branch when making code contributions!**

📝 License
----------

RRRS is open-sourced under the Apache-2.0 license. See the LICENSE file for more details.
