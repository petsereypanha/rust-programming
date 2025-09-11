pub fn data_type() {
    let x: i32 = -10;

    let y: f32 = 5.0;

    let f: bool = false;

    println!("x: {}, y: {}, f: {}", x, y, f);

    let c1 = 'a';
    let c2 = '5';
    let c3 = '\u{263A}';
    println!("c1 = {}, c2 = {}, c3 = {}", c1, c2, c3);

    let tup = (10, 'a', 10.5);
    let first_elem =tup.0;
    println!("{}", first_elem);

    let arr = [10, 20, 30];
    let num1 = arr[0];
    println!("num1 = {}", num1);

}