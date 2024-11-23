## Primitive Data Types
Considering Rust is a statically typed language and the data types are known at compile time, how does Rust handle data types?
Rust is a statically typed language, which means that the data types are known at compile time.    
This allows the compiler to catch type errors early on and ensure that the program is type-safe.    
Rust has a rich set of built-in data types, including integers, floating-point numbers, booleans, characters, strings, arrays, tuples, and more. 
**Rust also allows you to define your own custom data types using structs and enums.**


## Declaring a string type
To declare a variable as string, juist assign the string value to the variable:
```rust
let my_string = "Hello, World!";
```
This implcity sets the type of the variable to `&str` type.   

It is also possible to explicity declate the variable as a string type:
```rust
let my_string: &str = "Hello, World!";
```

## Declaring a character type
To explicity delare a variable as a character type, use the `char` keyword: 
```rust
let my_char: char = 'a';
```

## Declaring a boolean type
To declare a boolean variable, use the `bool` keyword:
```rust
let my_bool: bool = true;
```

## Declaring an unsigned integer of 8 bits
To explicity declare an unsigned 8 bit variable use keyword:
```rust
let my_uint: u8 = 0;
```
This can take : 2^8 - 1 as the maximum value => 255.   
Similary there is u16, u32, u128, u256. There needs to be careful consderation on what will be the maximum value that the variable will have in its lifetime.


