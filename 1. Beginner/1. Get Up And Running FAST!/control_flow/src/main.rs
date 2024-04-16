fn main() {
    let mut a = 10;
    if a > 5 {
        println!("a is greater than 5");
    } else if a < 5 {
        println!("a is less than 5");
    } else {
        println!("a is equal to 5");
    }

    let b: i32 = if a > 5 { 1 } else { 0 };
    println!("b is {}", b);

    // loops
    loop {
        println!("Infinite loop");
        break;
    }
    // labeling loops
    'outer: loop {
        println!("Infinite loop");
        loop {
            println!("Inner loop");
            break 'outer;
        }
    }

    while a < 15 {
        println!("a is {}", a);
        a += 1;
    }

    let a = [1, 2, 3, 4, 5];
    for i in a {
        println!("i is {}", i);
    }
}
