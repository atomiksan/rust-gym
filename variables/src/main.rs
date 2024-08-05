fn main() {
    let x = 5;

    let x = x + 1;
    println!("The value of x is {}", x);

    {
        let x = x + 2;
        println!("The value of x now is {}", x);
    }
    println!("The value of x now is {}", x);

    let spaces = "      ";
    let spaces = spaces.len();

    println!("the value of spaces is {spaces}");
}
