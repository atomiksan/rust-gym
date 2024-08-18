fn main() {
    println!("hello world!");
    another_function(5, 'h');
}
fn another_function(x: i32, y: char) {
    println!("another function!");
    println!(
        "the value of x & tag passed from main function is {}{}",
        x, y
    );
}
