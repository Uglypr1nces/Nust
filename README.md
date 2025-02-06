# NUST

NUST (Nifty User-friendly Simple Text-editor) is a basic text editor built using Rust and `egui`. It's designed to help you learn and explore Rust programming while creating a simple but functional text editor.

## Features

- **Text Editing**: Type and edit text in a clean, minimal interface.
- **Save to File**: Save your text to a file with a single click.
- **Load from File**: Load text from a file to continue editing.

## Getting Started

### Prerequisites

- Install [Rust](https://www.rust-lang.org/tools/install) on your system.

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/Uglypr1nces/Nust.git
   cd Nust
   ```
2. Build the project:
   ```bash
   cargo build --release
   ```
3. Run the project:
   ```bash
   cargo run
   ```

## Usage

1. Type your text in the main editing area.
2. To save:
   - Enter a file name (e.g., `myfile.txt`).
   - Click the **Save** button to save the text to the file.
3. To load:
   - Enter the file name of the saved file.
   - Click the **Load** button to load the content into the editor.

## File Structure

```
NUST/
├── src/
│   └── main.rs   # Main application code
├── Cargo.toml    # Dependencies and project configuration
└── README.md     # Project documentation
```

## Contributing

Contributions are welcome! If you have ideas or improvements, feel free to submit a pull request.

