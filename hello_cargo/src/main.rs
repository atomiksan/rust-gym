fn main() {
    println!("Hello, world!");
    let age = 27;
    let name = "Happy";

    println!("Hello {}! you are {} years old", name, age);

    if age >= 18 {
        println!("You are an adult");
    } else {
        println!("You are not an adult");
    }

    //looping
    for i in 1..5 {
        println!("Iteration Number: {}", i);
    }

    //Array
    let arr: [i32; 4] = [1, 2, 3, 4];
    for i in 0..4 {
        println!("Array Value: {}", arr[i]);
    }

    //tuples
    let tup = (500, "hi sena", true);
    println!("{}", tup.0);
    println!("{}", tup.1);
    println!("{}", tup.2);
}
