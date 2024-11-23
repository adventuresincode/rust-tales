# Reference and Borrowing
reference is nothing but a pointer
used with the `&` sign
It is actually pointing to the variable (in the stack) which is pointing to the value on the heap. 

This is all about pointers. using the `&` operator, the address of the variable can be got. This pointer can be used to reach the variable and in turn the value stored in the variable.   
Consider a variable called >name.   
declare and assign a value to this variable:
```rust
let name:&str ="Some name";
``` 

The address of this variable can be got and stored in another variable:
```rust
let name_ptr=&name;
```
Here ***name_ptr*** is borrowing the reference to ***name***.

The ***name_ptr*** should now point to the address of the ***name*** variable.   Printing out this value should display the contents of variable `name`:
```sh
"Some name"
```

When the program the program terminate, the variable `name` is dropped but the ***name_ptr*** is not dropped as it has no value. 

---
Reference does not own the pointer - it is called a borrowed reference. 
The variable which is assigned the value is the **smart pointer** and it points to value on the heap.

Since the reference does not own the value it cannot be used to change the value. 
Attempting to do so leads to a compile time error.   

To overcome this, make the variable mutuable. Then, borrow the value to a mutable pointer.   
Then, the most important step, use the pointer valriable's push command to append to the value in the heap.



-----
## Mutuable borrow
Immutuable borrows have follow the mutuable. 
This is because the all the mutations are expected to happen before the reference is borrowed. 

---
## Dereference
Tyhe operator to derefernce is `*`.   


