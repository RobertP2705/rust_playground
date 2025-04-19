fn main() {
    let mut x : i32 = 1;
    assert_eq!(x,1);

    x = 7;
    assert_eq!(x,7);
    let mut _x = x;
    _x += 3;

    let _y = 4;

    let _y = "I can also be bound to text";
    println!("Success!");
}
