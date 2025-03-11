# Dino_File_Content_Search

Dino_File_Content_Search is a high-performance, parallel file content search tool built with Rust. Designed for extreme efficiency, it leverages Tokio for asynchronous operations and optimally manages system resources.

## 🚀 Features
- **Blazing Fast Search**: Utilizes multi-threading for rapid file content search.
- **Efficient Resource Management**: Ensures optimal performance using a controlled number of concurrent searches.
- **Cross-Platform Support**: Works on Linux and macOS.
- **Customizable Limits**: Adjust system file descriptor limits for maximum efficiency.

## 🔧 Installation
1. **Clone the repository:**
   ```sh
   git clone https://github.com/natiqmammad/Dino_File_Content_Search.git
   cd Dino_File_Content_Search
   ```
2. **Install Rust** (if not installed):
   ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
3. **Build the project:**
   ```sh
   cargo build --release
   ```

## ⚡ Usage
Run the search tool by providing the directory path and search term:
```sh
./target/release/dino_search
     <directory>
     <search_term>
```

### Example:
```sh
./target/release/dino_search
     ~/Documents
       error log
```

## 🛠 System Optimization
To maximize performance, increase the file descriptor limit:
```sh
ulimit -n 65535
```
This ensures the tool can handle a vast number of files simultaneously without hitting system limits.

## 🦖 Future Enhancements
- Support for Windows
- Advanced filtering options
- Performance benchmarking tools

## 📜 License
MIT License. Feel free to contribute and enhance Dino_File_Content_Search!

