
fn main() {
    let vec = vec![1, 2, 3];

    let int_slice = &vec[..];
    println!("int_slice {}", int_slice);

    let str_slice: &[&str] = &["one", "two", "three"];
    println!("str_slice {}", str_slice);


}

