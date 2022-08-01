fn main() {
    println!("The maximum number in i8 {}", std::i8::MAX);
    println!("The maximum number in u8 {}", std::u8::MAX);

    //FLOAT
    let z: f64 = 3.65;
    println!("max number in f32: {}", std::f32::MAX);

    let status = false;
    println!("variable values {:?}", (z, status));

    let not_equals_test = 18 != 18;
    let equals_test = 18 == 18;
    println!(
        "the boolean evaluation {:?}",
        (not_equals_test, equals_test)
    );

    let mut x1 = 40;
    let x2;
    x1 = x1 * 3;
    x2 = x1 - 2;
    println!("value of x1, x2 are {:?}", (x1, x2));
}
