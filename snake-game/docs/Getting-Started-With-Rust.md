
AI Lecture - Additional information lecture - tips and tricks


Manage Rust versions
- main RUST website has info on the available versions

```
rust default nightly OR stable
```


-----
1. Create a project folder
2. Create the main.rs file - this is the entry point
3. fn 


----
### To create a simple REUST program that prints Hello-World
1. Create a file called main.rs
2. The entry point in main is the line
```
fn main()
```
3. The body of the function follows the main key word and starts with \{ and end with \}:
```
fm main(){

}
```
4. We can define and assign values to variable using the `let` keyword.   
For e.g. 
```
let a="something"
```
In our code to define a variable called greeting and assing it the value "Hello Word" should be like this:
```
fn main(){
    let greeting="Hello World";
}
```
The greeting variable now has the value "Hello World".

5. To print the line "Hello World", use the macro >println.   
The macro is a reusable function and the format of a macro looks like this:
```
macro-name!()
```
Note: The macro name is followed by a parenthesis.    
For the println:
```
println!("{}","line to the printed");
```
The 1st parameter in println is a template and the second parameter is the string that replaces the template.

Now our code in the main.rs should look like this:
```
fn main(){
    let greeting="Hello World"
    println!("{}", greeting);
}
```
This print line should print the message `Hello World` on the console.

6. To compile the code in the command line, run:
```
rustc main.rs
```
LLVM is used to compile the Rust code.
This should result in a file called `main` being generated which can be executed:
```
./main
```
-----

## The 'mut' keyword
By default the variable are immutable 
To explecity make it mutable, you need to  use the key word `mut`


---
## Cargo
Cargo is the package manager for Rust. The initial install should have already installed Cargo.   
The packages in Rust are called `Crates`.   

To start using cargo, initialize it in the source directory using the command:
```
cargo init
```

This should result in the creation of the manfiest file called >Cargo.toml.   
This is the file that is like `package.json` for node. It contains the list of all the packages that will be used to build and run the application.  

To run the main rust file using cargo, just state:
```
cargo build 
cargo run
```

This should initially build the file and then run the executable.




