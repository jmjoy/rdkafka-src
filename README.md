# rdkafka-src

[![Rust Version](https://img.shields.io/badge/rust-1.85%2B-orange.svg)](https://www.rust-lang.org)
[![GitHub Actions CI](https://img.shields.io/github/actions/workflow/status/jmjoy/rdkafka-src/ci.yml?branch=master&label=CI&logo=github)](https://github.com/jmjoy/rdkafka-src/actions)
[![crates.io](https://img.shields.io/crates/v/rdkafka-src.svg)](https://crates.io/crates/rdkafka-src)
[![Downloads](https://img.shields.io/crates/d/rdkafka-src.svg)](https://crates.io/crates/rdkafka-src)
[![docs.rs](https://img.shields.io/docsrs/rdkafka-src?logo=rust)](https://docs.rs/rdkafka-src)
[![License](https://img.shields.io/crates/l/rdkafka-src?color=blue)](https://github.com/jmjoy/rdkafka-src/blob/master/LICENSE)

## Overview

**rdkafka-src** is a Rust project designed to simplify the integration of the [librdkafka](https://github.com/confluentinc/librdkafka) library into your Rust projects. It is intended for use in your `build.rs`, allowing you to select and configure the desired version of librdkafka with ease.

## Prerequisites

Before building **rdkafka-src**, ensure that you have the following binary tools installed on your system:

- **GCC or Clang:** Required for compiling native code.
- **Make:** Utilized by the build system to compile and link the library.
- **Git:** Necessary for fetching the source code and managing repository dependencies.

Make sure these tools are available in your system's PATH for a smooth build process.

## Features

- **Build Script Integration:** Seamlessly integrates with `build.rs` for automated build processes.
- **Version Selection:** Easily choose the librdkafka version to build and link against.
- **Customizable Build:** Configure additional build options through environment variables and build script parameters.

## Installation

Add **rdkafka-src** as a dependency in your project's `Cargo.toml` file. If published on [crates.io](https://crates.io), you can add it as follows:

```shell
cargo add rdkafka-src --build
```

## License

This project is licensed under the [MulanPSL-2.0](https://spdx.org/licenses/MulanPSL-2.0.html) license.
