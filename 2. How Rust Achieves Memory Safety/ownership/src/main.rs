fn main() {
    let s1: String = String::from("Rust");
    {

        let s2: String = String::from("Rust 2");
    }//s2 is dropped

    // let s3 = s1;
    let s3 = s1.clone();
    
    let s4 = generate_string();
    print_string(s1.clone());

    let s5 = add_to_string(s1);
    //print_string(s1); //ownership of s1 moved into p1

    

    // println!("s1 is: {}", s1); this will throw error as theres no s1, we dont get any error if we clone
    println!("s1 is: {}", s1);
    println!("s4 is: {}", s4);
    println!("s5 is: {}", s5);
}//s1 is dropped

fn generate_string() -> String {
    let s: String = String::from("Rust 4");
    s
}//s is dropped
fn print_string(p1: String) {
    println!("{}", p1);
}//p1 is dropped

fn add_to_string(mut p1: String) -> String {
    p1.push_str(" is awesome!");
    p1
}