# RocketOne - A Rust Learning Project

## 🎯 Project Goal

This project aims to create a web application using the **Rust Rocket framework**. The application provides a single webpage that displays detailed system information about the Mac it's running on, similar to PHP's `phpinfo()` function. It collects and presents as many system details as possible in a clear and organized manner.

**This project serves primarily as a learning tool for Rust and the Rocket framework.**

## 🚀 What's Been Implemented

✅ Basic Rocket web server setup  
✅ System information collection and display  
✅ Template rendering with Tera  
✅ Three main routes:
- `/` - Welcome page
- `/info` - System information (OS, CPU, memory, hostname)
- `/env` - Environment variables

## 🛠️ Technology Stack

* **Language:** Rust (2024 edition)
* **Web Framework:** Rocket 0.5.0
* **Template Engine:** Tera (via rocket_dyn_templates)
* **System Info:** sysinfo crate
* **Development Environment:** Any Rust-compatible IDE
* **Target Platform:** macOS (but works on other platforms too)

## 📋 Features

1. **System Information Display:**
   - Operating system name and version
   - Kernel version
   - CPU model and core count
   - Total memory
   - Hostname

2. **Environment Variables:**
   - Complete list of system environment variables

3. **Template-Based Rendering:**
   - Clean HTML output using Tera templates
   - Responsive design with organized data presentation

## 🔧 Prerequisites & Installation

### Installing Rust

Before you can run this project, you need to have Rust installed on your system.

#### Option 1: Install via rustup (Recommended)

```bash
# Install Rust using the official installer
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Restart your terminal or source the environment
source ~/.cargo/env

# Verify installation
rustc --version
cargo --version
```

#### Option 2: Using Homebrew (macOS)

```bash
# Install Rust via Homebrew
brew install rust

# Verify installation
rustc --version
cargo --version
```

### Additional Requirements

- **Git:** For cloning the repository
- **Terminal/Command Line:** For running cargo commands

## 🚀 Getting Started

### 1. Clone the Repository

```bash
git clone https://github.com/Etschmia/rocketone.git
cd rocketone
```

### 2. Build the Project

```bash
# Download dependencies and build
cargo build
```

### 3. Run the Application

```bash
# Start the development server
cargo run
```

The application will start on `http://localhost:8000` by default.

### 4. Access the Application

Open your web browser and navigate to:

- **Homepage:** `http://localhost:8000/`
- **System Info:** `http://localhost:8000/info`
- **Environment Variables:** `http://localhost:8000/env`

## 📁 Project Structure

```
rocketone/
├── src/
│   └── main.rs              # Main application code
├── templates/               # Tera template files
│   ├── base.html.tera       # Base template
│   ├── index.html.tera      # Homepage template
│   ├── info.html.tera       # System info template
│   └── env.html.tera        # Environment variables template
├── Cargo.toml              # Project dependencies
├── Rocket.toml             # Rocket configuration
└── README.md               # This file
```

## 🧑‍💻 Learning Focus

This project is designed for developers who want to learn Rust, particularly those with experience in:
- **C/ANSI C**
- **PHP**
- **Python**

### Key Rust Concepts Demonstrated

1. **Ownership and Borrowing:** How Rust manages memory safely
2. **Pattern Matching:** Using `match` and `if let` constructs
3. **Error Handling:** Using `Result` and `Option` types
4. **Structs and Implementations:** Organizing data and behavior
5. **Macros:** Using Rocket's route macros (`#[get]`, `#[launch]`)
6. **Crate System:** Managing dependencies with Cargo
7. **Serialization:** Using Serde for data conversion

### Code Documentation

The code is extensively commented to explain:
- Each important Rust concept
- Library (crate) usage
- Design decisions
- Learning points for newcomers to Rust

## 🔧 Development Commands

```bash
# Run in development mode (with auto-reload)
cargo run

# Build for release
cargo build --release

# Run tests (if any)
cargo test

# Check for compilation errors without building
cargo check

# Format code
cargo fmt

# Run clippy (Rust linter)
cargo clippy
```

## 📚 Dependencies

- **rocket:** Web framework for Rust
- **sysinfo:** Cross-platform system information library
- **rocket_dyn_templates:** Dynamic template support for Rocket
- **serde:** Serialization/deserialization framework

## 🎓 Learning Resources

If you're new to Rust, these resources might help:

- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rocket Programming Guide](https://rocket.rs/guide/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)

## 🤝 Contributing

This is a learning project, but contributions that help improve the educational value are welcome! Feel free to:

- Add more system information displays
- Improve the template design
- Add comments explaining Rust concepts
- Create additional learning examples

## 📝 License

This project is created for educational purposes. Feel free to use it for learning Rust and web development.

---

**Happy Learning! 🦀**