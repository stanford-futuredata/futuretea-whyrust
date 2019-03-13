
fn return_a_pointer<'a>(left: bool,
                    x: &'a Vec<i32>,
                    y: &'b Vec<i32>) -> &'a i32 {
    if left {
        return &x[0]
    } else {
        return &y[0]
    }
}


fn main() {
    println!("Hello, world!");
}
