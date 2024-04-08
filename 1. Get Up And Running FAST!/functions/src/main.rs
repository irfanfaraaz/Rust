fn main() {
    println!("Hello, world!");
    my_function(5);
    let b:i32 = my_function(6);
    println!("The value of b is: {}", b);
}

fn my_function(x:i32)->i32{
    println!("The value of x is: {}", x);
    let y:i32 = 10;
    y
}