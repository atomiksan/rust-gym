fn main() {
    let mut nums = vec![1, 2, 3];
    nums.push(4);
    println!("{:?}", nums);
    nums.pop();
    println!("{:?}", nums);

    let mut vect: Vec<&str> = Vec::new();
    vect.push("hi");
    vect.push("mum");
    println!("{:?}", vect);

    vect.reverse();
    println!("{:?}", vect);

    let vec = Vec::<i32>::with_capacity(2);
    println!("{}", vec.capacity());

    let v: Vec<i32> = (0..5).collect();
    println!("{:?}", v);
}
