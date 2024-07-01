# text-processor [WIP]

TextProcessor is a Rust project designed to process various text files. The project aims to provide a flexible and efficient solution for handling and transforming text files.

## Features

- SRT File Processing: Reads .srt files from a specified input directory and copies them to an output directory.
- Markdown Combining: Combines the contents of processed .srt files into a single Markdown file.
- Command-line Interface: Allows users to specify input and output directories via CLI arguments, with default values if not specified.
- Future Expansion: Designed with extensibility in mind to support various other text file formats in the future.

## Getting Started

### Prerequisites

- Rust (version 1.50 or later)

### Installation

Clone the repository and navigate to the project directory:

```shell
git clone https://github.com/Sunny-117/text-processor
cd text-processor
```

### Usage

You can run the project with default input and output directories (input and output):

```shell
cargo run
```

### To specify custom directories for input and output:

```shell
cargo run -- --input custom_input_dir --output custom_output_dir
```

### Test CLI

```shell
cargo install --path .
tp
tp --input custom_input_dir --output custom_output_dir
```

## Directory Structure

```
text-processor/
├── Cargo.toml
├── src/
│   ├── main.rs
│   └── file_ops.rs
├── input/
│   ├── file1.srt
│   ├── file2.srt
│   └── ...
└── output/  (generated after running the program)
```

## Contributing

Contributions are welcome! Please fork the repository and open a pull request with your changes. Ensure your code adheres to the project's coding standards and includes appropriate tests.

## License

This project is licensed under the [MIT License](/LICENSE). See the LICENSE file for details.

## Acknowledgements

Special thanks to the Rust community for their support and contributions.
