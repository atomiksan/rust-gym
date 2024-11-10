fn main() {
    let mut val = 1;

    // if statements
    if val > 1 {
        println!("val is greater than 1");
    } else if val == 1 {
        println!("val is equal to 1");
    } else {
        println!("val is not equal to 1")
    }

    //loops
    let mut num = 0;
    'counter: loop {
        println!("Counter : {}", num);
        let mut dec = 5;
        loop {
            println!("Decreasing: {}", dec);
            if dec == 4 {
                break;
            }
            if num == 2 {
                break 'counter;
            }
            dec -= 1;
        }
        num += 1;
    }

    // while loop
    val = 0;
    while val <= 5 {
        println!("Num: {}", val);
        val += 1;
    }

    let v: Vec<i32> = (0..10).collect();
    for ele in v {
        println!("{}", ele);
    }

    for numb in (0..10).rev() {
        println!("{}", numb);
    }

    control_flow(5);
}

fn control_flow(val: i32) {
    if val == 1 {
        println!("Equals to 1");
    } else if val > 50 {
        println!("Greater than 50");
    } else if val > 25 && val < 50 {
        println!("Greater than 25 but less than 50");
    } else {
        println!("Less than 25");
    }
}
