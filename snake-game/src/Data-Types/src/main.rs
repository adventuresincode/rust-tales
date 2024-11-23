fn main() {
    let greeting = "Hello, world!";

    let ret= greet(greeting);
    if ret == 0 {
        println!("Success");
    }
    else {
        println!("Error: {}", "Failed");
    }
}

fn greet(msg: &str) -> u8   {
    println!("{}", msg);
    return 0;
}
