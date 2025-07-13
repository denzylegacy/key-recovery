<div align="center">
  <h1>Key Recovery</h1>

[![Rust](https://img.shields.io/badge/Rust-stable-orange.svg)](https://www.rust-lang.org/)
[![Egui](https://img.shields.io/badge/Egui-0.28.0-blue.svg)](https://github.com/emilk/egui)
[![Anyhow](https://img.shields.io/badge/Anyhow-1.0.86-red.svg)](https://github.com/dtolnay/anyhow)
[![Webbrowser](https://img.shields.io/badge/Webbrowser-0.8.0-purple.svg)](https://github.com/amodm/webbrowser-rs)

A simple and user friendly cross-platform desktop application built with Rust and Egui to recover Windows Activation Keys on both Linux and Windows operating systems.

</div>

## Overview

This application provides a straightforward graphical interface to retrieve the embedded Windows product key from your system. It supports both Windows and Linux, adapting its key recovery mechanism to the underlying operating system.

## Project Website

Visit the official project website: [https://keyrecovery.lovable.app/](https://keyrecovery.lovable.app/)

## Features

*   **Cross-Platform:** Works seamlessly on Windows and Linux.
*   **Simple UI:** A clean and intuitive user interface for easy key recovery.
*   **Copy & Save:** Options to copy the recovered key to clipboard or save it to a text file.
*   **Donation Support:** A convenient link to support the developer.

## Technologies Used

*   **Rust:** The primary programming language for building a robust and performant application.
*   **Egui:** A simple, fast, and highly portable immediate mode GUI library for creating the user interface.
*   **Anyhow:** A flexible error handling library for Rust applications.
*   **Webbrowser:** Used for opening external links (like the donation address) in the default web browser.
*   **WMI (Windows-specific):** Windows Management Instrumentation is used on Windows to query system information, including the product key.
*   **Regex (Linux-specific):** Regular expressions are used on Linux to parse the product key from system files.

## Installation and Usage

### Prerequisites

*   Rust toolchain (rustup recommended)

### Build and Run

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/your-username/key-recovery.git # Replace with your actual repo URL
    cd key-recovery
    ```
2.  **Build the application:**
    ```bash
    cargo build --release
    ```
3.  **Find the compiled binary:**
    The compiled executable will be located in the `target/release/` directory.

4.  **Run the application:**

    *   **On Windows:**
        Navigate to `target/release` and run `key-recovery.exe`.

    *   **On Linux (for key recovery):**
        ```bash
        chmod +x target/release/key-recovery
        sudo ./target/release/key-recovery
        ```
        *Note: Root privileges are required to access the system's product key information.*

    *   **On Linux (for general use/donation link):**
        ```bash
        ./target/release/key-recovery
        ```
        *Note: Maybe the donation link will not open correctly when run with `sudo` due to browser security policies.*

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
