# GTrace - Gerber to G-Code Converter

A high-performance, minimalist desktop application designed to convert standard Gerber files (.GBL/.GBR) into ready-to-machine G-Code for custom PCB laser engraving.

![GTrace UI](https://via.placeholder.com/800x450.png?text=Put+Your+Software+Screenshot+Here)

## Core Features

- **High-Performance Processing:** The core engine is built from scratch in Rust, ensuring safe and lightning-fast geometry processing, parsing, and Boolean Union operations.
- **Smart Modal State Parsing:** Flawlessly reads EasyEDA Gerber formats, perfectly interpreting complex modal states and pin dimensions.
- **Automatic X-Axis Mirroring:** Built-in mirroring via bounding-box calculations for accurate bottom-layer PCB etching.
- **Minimalist UI:** A clean, monochromatic user interface built with modern .NET WPF, designed to reduce cognitive load and prevent human error.
- **Ready-to-Machine Output:** Generates standard G-Code instantly compatible with CNC laser software (verified via NC Viewer).

## Technology Stack

- **Core Engine:** Rust (`C-compatible DLL`, Computational Geometry)
- **Front End:** C# / .NET 8 WPF
- **Deployment:** Inno Setup (Single Executable Installer)

## Installation

You can install GTrace directly on your Windows machine:
1. Go to the [Releases](../../releases) page.
2. Download the latest `GTrace_Setup_v1.1.0.exe`.
3. Run the installer and follow the on-screen instructions.

## How to Use

1. Launch **GTrace** from your desktop.
2. Click **Browse...** to select your target Gerber file (`.GBL` or `.GBR`).
3. Click **Save As...** to define the output destination for your G-Code file.
4. Set your desired **Feed Rate** (mm/min).
5. Toggle **Mirror X-Axis** if you are processing a bottom-layer trace.
6. Click **GENERATE G-CODE**.
7. Your G-Code is ready to be sent to your CNC laser machine!

## Development Setup

If you want to build this project from the source:

### Prerequisites
- [Rust Toolchain](https://www.rust-lang.org/tools/install)
- [.NET SDK](https://dotnet.microsoft.com/download)

### Build Instructions
1. **Build the Core Engine:**
   ```bash
   cd Core_Engine
   cargo build --release

### Author
**Ken Panya Trirpom**
