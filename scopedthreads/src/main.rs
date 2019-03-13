
fn main() {
    use crossbeam_utils::thread;

    let var = vec![1, 2, 3];

    // Makes sure that the thread wrapped in here don't die before the main thread.
    thread::scope(|s| {
        for _ in 0..5 {
            s.spawn(|_| {
                println!("thread borrowing `var`: {:?}", var);
            });
        }
    }).unwrap();
}
