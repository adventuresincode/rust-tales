## Functions and parameters

1. To create a function, use the keyword `fn`. The function body is in the parenthesis.
```rust
fn greet(){}
```
2. To pass parameters to a function, the parameters needs to be declared in the function definition and within the parenthesis.   
To pass a string variable, called *msg* to the **greet** function, in the parenthesis add: 
```rust
fn greet(msg: &str){

} 
``` 
3. To return a value, use the arrow operator `->`.  
Whatever follows the arrow operator is the return value.    
To return an unsigned integer from the **greet** function:
```rust
fn greet(msg: &str) -> u8{
    //the body of the function
}
```

4. To return a value, using the `return` keyword.
```rust
fn greet(msg: &str) ->{
    return 0;
}
``` 
