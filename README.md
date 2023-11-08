
<img src="https://github.com/edizeqiri/RustyFiles/assets/89740646/76ce2779-f0db-4f98-94c9-aa2ad7b261d7" width="400">

# RustyFiles

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

