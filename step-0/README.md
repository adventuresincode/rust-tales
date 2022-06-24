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

On Ubuntu:
Ensure to run the following commands
```
$sudo apt-get update
$sudo apt install build-essential
```

## 2. Editor
Visual Studio Code: Install Rust Extension for Visual Studio Code
In Visual Studio code, install the following extensions:
1. Rust support for Visual Studio Code
1. rust-analyzer


## 3. Hello RUST World

In visual studio code (or the editor of your choice), create a file named:   
**hello-world.rs**


Adding the following lines in the file:
```
fn main(){
    println!("Hello, RUST World!!!");
}
```    

! Remember the semi-colon at the end of the print line!    

compile the code:    
```
$rustc hello-world.rs
```

Run the executable:    
```
$hello-world
```


4. Cargo 

While the above is a fairly simple file, in the future when working with complex files, then the **cargo** package manager is handy. Execute the following steps to get a hang of cargo:
i. Initialize the folder:     
```
$cargo new hello_cargo_world
```  
This should result in the creation of a folder called "hello_cargo_world".


The **Cargo.toml** is where all the dependencies are located.   

ii. To execute this new program, navigate in to the new folder and execute the cargo run:   
```
$cd hello_cargo_world
$cargo run
```   

This will compile and execute the code!!!