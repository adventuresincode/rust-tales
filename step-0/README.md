Installing RUST

## 1. Installing RUST
On WSL running on Windows 11 (or Ubuntu)
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```   
The installation options to choose from…Select 1 and proceed.
Notification on successfully installing RUSTOn WSL, remember to configure the current shell by running:   
```
$source $HOME/.cargo/env
```
Validating the installation:
On the terminal run   
```
$rustc --version
```

This should return the rust version.    
```
$cargo --version
```
This should return the current version of "cargo" which is the package manager for RUST.    
Wrap up the installation by running the update:    
```
$rustup update
```

## 2. Editor
Visual Studio Code: Install Rust Extension for Visual Studio Code
In Visual Studio code, install the following extensions:
1. Rust support for Visual Studio Code
1. rust-analyzer
