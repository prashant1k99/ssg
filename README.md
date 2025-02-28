# Static Site Generator (SSG)

This is the practice project for building a static site based on Coding Challenges. The SSG project is designed to help you generate static websites by processing Markdown files and rendering HTML templates.

## Table of Contents

1. [Introduction](#introduction)
2. [Features](#features)
3. [Installation](#installation)
4. [Usage](#usage)
5. [Configuration](#configuration)
6. [Creating Custom Themes](#creating-custom-themes)
7. [Future Enhancements](#future-enhancements)
8. [Contributing](#contributing)
9. [License](#license)
10. [Contact Information](#contact-information)

## Introduction

The Static Site Generator (SSG) project is a tool for generating static websites from Markdown files. It is built using Rust and provides a simple and efficient way to create static sites.

## Features

- Converts Markdown files to HTML
- Supports custom themes
- Fast and efficient build process
- Easy-to-use configuration

## Installation

The SSG project is available through GitHub Releases. To install the latest version, follow these steps:

1. Go to the [Releases page](https://github.com/prashant1k99/ssg/releases) of the repository.
2. Download the appropriate binary for your operating system.
3. Extract the binary and place it in a directory that is included in your system's PATH.

Alternatively, you can build the project from source if you have Rust installed on your machine. If you don't have Rust installed, you can download it from [rust-lang.org](https://www.rust-lang.org/).

1. Clone the repository:

   ```sh
   git clone https://github.com/prashant1k99/ssg.git
   ```

2. Navigate to the project directory:

   ```sh
   cd ssg
   ```

3. Build the project:

   ```sh
   cargo build --release
   ```

## Usage

To generate a static site, follow these steps:

1. Place your Markdown files in the `content` directory.
2. Update the `config.toml` file with your site settings.
3. Run the build command:

   ```sh
   cargo run --release
   ```

The generated site will be placed in the `dist` directory.

## Configuration

The `config.toml` file is used to configure the SSG project. Here is an example `config.toml` file:

```toml
[settings]
site_title = "My Static Site"
site_description = "A static site built with SSG"
theme = "default"
out_dir = "dist"
asset_dir = "static"
current_year = 2025

[custom]
# Add any custom settings here
```

## Creating Custom Themes

To create a custom theme, follow the steps outlined in the [Theme Building Guide](docs/theme-building.md).

## Future Enhancements

- [ ] Parallel processing of the build
- [ ] Improved CI/CD scripts using GitHub Actions
- [ ] Incremental builds (only build changed or new files)
- [ ] Development server with hot reloading
- [ ] Plugin system for extending functionality
- [ ] Theme marketplace for sharing and discovering custom themes

## Contributing

Contributions are welcome! If you would like to contribute to the project, please follow these steps:

1. Fork the repository
2. Create a new branch (`git checkout -b feature-branch`)
3. Make your changes
4. Commit your changes (`git commit -m 'Add new feature'`)
5. Push to the branch (`git push origin feature-branch`)
6. Create a pull request

Please make sure to update tests as appropriate.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contact Information

For further assistance, you can reach out to the project maintainer:

- **Name**: Prashant Singh
- **GitHub**: [prashant1k99](https://github.com/prashant1k99)
