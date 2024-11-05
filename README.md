# `Dockerdump`- extract files from Docker images

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/v/dockerdump)](https://crates.io/crates/dockerdump)
[![Rust](https://github.com/saefstroem/dockerdump/actions/workflows/rust.yml/badge.svg)](https://github.com/saefstroem/dockerdump/actions/workflows/rust.yml)

Dockerdump is an interactive tool for fetching and extracting files from Docker container images. It allows you to download images from Docker Hub or custom registries and browse their contents with an intuitive interface.

Suppose there is some binary that you know is compatible with your system, but the building process is complex or time-consuming. You can use Dockerdump to extract the binary from a Docker image and run it on your machine without the overhead of using Docker or dealing with the docker daemon.

## Features

- Interactive file browser with directory navigation
- Support for both Docker Hub and custom registries
- Multi-file selection capabilities

## Installation

### Prerequisites

- Rust toolchain (1.56 or later)
- Cargo package manager

### Steps

#### Option 1: Install from Crates.io

```bash
cargo install dockerdump
```

#### Option 2: Build from Source

1. Clone the repository:
```bash
git clone https://github.com/saefstroem/dockerdump.git
cd dockerdump
```

2. Build the project:
```bash
cargo build --release
```

3. Run the binary:
```bash
./target/release/dockerdump
```

## Usage

1. Launch the application via the terminal:
    ```bash
    dockerdump
    ```
2. Enter a Docker image tag (e.g., `ubuntu:latest`)
3. Choose between Docker Hub or a custom registry
4. Navigate through the extracted files using:
   - Arrow keys to move up/down
   - Space to select files
   - Enter to confirm selection
   - Select "[Done browsing]" to exit

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Security Note

Dockerdump extracts files with controlled permissions (666) for safety. Be cautious when exploring untrusted container images.

## Support my work

- `kaspa:qphqua5wghak5umtmv8h9cdkfg46up42wjwr0eam528htx945ymdz9sa0fvpn`
- `0x0d96F733969ED1ae8C58Bcad6F6B11993B051cC4`

---

**Note**: Always verify the source and content of Docker images before using them in production environments.