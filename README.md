# RUST Tutorial

## Recommended
If you are VScode as your IDE, install **rust-analyzer** extension.

## Covered Topics

### 1) 15/03/2024 - hello world
A simple hello world program.

### 2) 15/03/2024 - cargo
**Cargo** is a package manager, debugger, tester and also a build tool for rust projects.

- Create a new rust project ```cargo new <project-name>```
- Build the project ```cargo build```
- Build and run the project ```cargo run```
- Run tests ```cargo test```

Every cargo project contains a ```Cargo.toml``` file as a manifest file of the project. This file contains the project related meta data like project name, version, dependencies, etc,.

If you see ```Cargo.lock``` file in your project, it is an auto-generated file by cargo. we don't touch this file.