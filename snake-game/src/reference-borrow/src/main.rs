fn main() {
    let name="Some Name";

    let name_ptr=&name;
    println!("Name-ptr: {}", name_ptr);
    borrow_sample();
    dereference_sample();
}

fn borrow_sample() {
    //declaring the variable as mutable
    let mut message= String::from("Hello");
    //borrowing the value
    let message_2: &mut String= &mut message;
    message_2.push_str(" World");
    println!("Value-ptr: {}", message_2);
    println!("Value: {}", message);
    
}

fn dereference_sample() {
    let a=10;
    let a_ptr=&a;
    let a_ptr2=&a_ptr;

    //dereferencing the pointer
    let a_val=*a_ptr;
    println!("Value: {}", a_val);

    //dereferencing the pointer
    let a_val2=**a_ptr2;
    println!("Value: {}", a_val2);
}
 
 
 
 
