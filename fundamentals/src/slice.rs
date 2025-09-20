pub fn slice() {
    let s = String::from("Gullu Seth");
    let w1 = &s[0..5];
    let w2 = &s[6..10];
    println!("{}: {}", w1, w1.len());
    println!("{}: {}", w2, w2.len());
    let s = String::from("Gullu Seth");
    let w1 = get_slice(&s);
    println!("{}", w1);
}

fn get_slice(s : &String) -> &str {
    return &s[0..6];
}