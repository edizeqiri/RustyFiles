<p align="center"><img width=50% src="https://github.com/edizeqiri/RustyFiles/assets/89740646/33d25f79-d0dd-4e7d-9a06-407fc123f588"></p>

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
![Python](https://img.shields.io/badge/python-v3.6+-blue.svg)
[![Build Status](https://travis-ci.org/anfederico/clairvoyant.svg?branch=master)](https://travis-ci.org/edizeqiri/RustyFiles)
![Dependencies](https://img.shields.io/badge/dependencies-up%20to%20date-brightgreen.svg)
[![GitHub Issues](https://img.shields.io/github/issues/anfederico/clairvoyant.svg)](https://github.com/edizeqiri/RustyFiles/issues)
![Contributions welcome](https://img.shields.io/badge/contributions-welcome-orange.svg)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/MIT)

## Basic Overview
RustyFiles is a high-performance, Rust-based file explorer designed to outpace and outperform traditional file management systems like Windows Explorer. With a focus on providing detailed statistics about file and folder age, size, and utility, RustyFiles helps users effectively manage their data, prioritize cleanup, and optimize storage effortlessly.

## Features

- **Fast and Efficient Browsing**: Navigate through your files faster than ever with Rust's performance and safety.
- **Statistical Insights**: Get detailed statistics about file and folder sizes, last accessed dates, and more.
- **Storage Optimization**: Identify and manage old or large files that are taking up unnecessary space.
- **User-Friendly Interface**: Enjoy a clean and intuitive UI that makes file management a breeze.

## Getting Started

```rust
git clone https://github.com/edizeqiri/RustyFiles
cargo run --color=always --package RustyFiles --bin RustyFiles 
```

### Prerequisites

```rust
[dependencies]
walkdir
inquire
log
env_logger
winapi 
```

