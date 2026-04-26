<div align="center">
  <img src="images/ahtapot.png" width="120" alt="Ahtapot logo" />
  <h1>Ahtapot</h1>
  <p>A CLI tool for bulk image renaming, resizing, and format conversion.</p>

  ![GitHub Stars](https://img.shields.io/github/stars/kaanboraoz/ahtapot?style=flat-square)
  ![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)
  ![Rust](https://img.shields.io/badge/rust-stable-orange?style=flat-square)
</div>

---

## Usage

```bash
aht --path ./images --name photo --width 800 --height 600 --locate ./output
```

| Flag | Short | Description |
|------|-------|-------------|
| `--path` | `-p` | Source directory containing images |
| `--name` | `-n` | Base name for output files |
| `--width` | `-w` | Target width in pixels |
| `--height` | `-e` | Target height in pixels |
| `--locate` | `-l` | Output directory for processed images |

## Installation

```bash
cargo build --release
```

## License

MIT

## Note

This is my first Rust project, built after a long break from programming.
Contributions and feedback are welcome.