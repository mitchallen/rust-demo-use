fn main() {
    println!("Hello, world!");

    let a = 25;
    let b = 32;
    let x = rust_lib_01::add(a, b);
    println!("Result: {} + {} = {}", a, b, x);

    let c = 25;
    let d = 20;
    let y = rust_lib_01::sub(c, d);
    println!("Result: {} + {} = {}", c, d, y);

    let mut bool_vector: Vec<bool> = Vec::new();
    for _ in 0..10 {
        bool_vector.push(rust_lib_01::flip());
    }
    println!("{:?}", bool_vector);
}
