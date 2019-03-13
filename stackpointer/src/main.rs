
/// This won't work!
/// 
/// Return a `Box<i32>`, which heap-allocates data.
fn return_a_stack_pointer() -> &'static i32 {
    let x = 2;
    let y = &x;
    return y;
}

fn main() {
    println!("Hello, world!");
}
