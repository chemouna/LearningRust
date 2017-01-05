
use std::collections::HashSet;

fn main() {
    let mut a: HashSet<i32> = vec!(1i32, 2, 3).into_iter().collect();
    let mut b: HashSet<i32> = vec!(2i32, 3, 4).into_iter().collect();

    assert!(a.insert(4));
    assert!(a.contains(&4));

    assert!(b.insert(4), "Value 4 is already in set b");
    b.insert(5);

    println!("A: {:?}", a);
    println!("B: {:?}", b);

}
