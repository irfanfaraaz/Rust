fn main() {
    // create
    let a: i32 = 10;
    println!("a is: {a}");

    //mutability
    let mut b: i32 = 20;
    b = 30;
    println!("b is: {b}");

    // shadowing
    let c: i32 = 40;
    let c: i32 = 50;
    println!("c is: {c}");

    //scope
    let d: i32 = 60;

    {
        let d: i32 = 70;
        println!("inner d is: {d}");
    }
    
    println!("d is: {d}")
}
