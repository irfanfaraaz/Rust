fn main() {
    let mut v = Vec::new();
    v.push(String::from("One"));
    v.push(String::from("Two"));
    v.push(String::from("Three"));

    let v2 = vec![1, 2, 3];

    // let s = &v[0]; // can panic
    // // let s = v.remove(0);

    for s in &mut v {
        s.push_str("!");
    }
    // let s = v.get(0);

    // if let Some(e) = s {
    //     println!("{}", e);
    // }

    for s in &v {
        println!("hi {}", s);
    }

    let mut v3 = vec![];
    let mut i = 0;

    for s in v {
        v3.push(s);
        println!("hello {}", v3.get(i).unwrap());
        i += 1;
    }
}
