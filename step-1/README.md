# Rust Macros

This is as simple as '!'.
To execute a macro in RUST, use the command followed by '!'.  
This results in **meta-programming** or the macro generating other code which will be substituted at a later point before the code is executed.


Variables
RUST Is statically typed and will need to know the variable type at compile time.
```
let a:i64=10;
```

Signed integers and Unsigned integers

Signed
 represented as -> i8, i16, i32, i64
 range          -> -2^(i-1)-1 to  2^(i-1)-1    

 for e.g., if i=8 then -> 2^(8-1)-1 to  2^(8-1)-1 = -127 to 127   

 to print this we could use the *std* library. This lubrary is included by default.


To use variables for decimal :
f32 and f64

For boolean variablesL
e.g. status=false;

---
Cleaner way is to use cargo 
1. cargo new [name of the project] - this results in a new folder in the locatoin 
2. under the folder creates in step 1, a src folder is creates which contains the main.rs. All the code goes to this file.
3. cd to the folder and run 
```
cargo build
```
4. run the app by 
```
cargo run
```
---

1. Using {:?}
This can be used for printing complex types.


---   
In rust, mut represents variable that is mutable, unlike the default characteristic of rust variable that makes in immutable.





