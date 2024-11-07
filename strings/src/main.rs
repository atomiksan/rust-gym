fn main() {
    let name = String::from("Satya");
    let crush = "Sena".to_string();
    let new_name = name.replace("Satya", "Atomik");

    println!("{}", name);
    println!("{}", crush);
    println!("{}", new_name);

    // &str = "string slice"

    let str = "hello";
    let str2 = str;
    let str3 = &str;

    println!("{}", str);
    println!("{}", str2);
    println!("{}", str3);

    // comparing strings
    println!("{}", "ONE".to_lowercase() == "one");

    let str4 = str2.to_owned() + str3;
    println!("{}", str4);

    let rust = "\x52\x75\x73\x74";
    println!("{}", rust);
}
