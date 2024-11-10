fn main() {
    println!("hello world!");
    another_function(5, 'h');
    print_details("Mario likes chocolates");
    println!("{}", gcd(20, 5));
    println!("{}", multiple_return_values(true));
    println!("{}", multiple_return_values(false));
    let s = "Hello".to_string();
    println!("{}", concat_string(&s));
}
fn another_function(x: i32, y: char) {
    println!("another function!");
    println!(
        "the value of x & tag passed from main function is {}{}",
        x, y
    );
}
fn print_details(s: &str) {
    println!("{}", s);
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let remainder = a % b;
        a = b;
        b = remainder;
    }
    a
}

fn multiple_return_values(flag: bool) -> bool {
    if flag == true {
        true
    } else {
        false
    }
}

fn concat_string(s: &str) -> String {
    let s2 = " World".to_string();
    s.to_owned() + &s2
}
