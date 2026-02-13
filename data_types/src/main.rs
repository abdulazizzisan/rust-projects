use std::io;

fn main() {
    let mut tup = (1, 2, 3);
    tup.1 = 20;
    println!("The value of tup is: {:?}", tup);

    let arr = [5; 3];
    println!("The value of arr is: {:?}", arr);

    let arr2 :[i32; 5];

    arr2 = [1, 2, 3, 4, 5];
    println!("The value of arr2 is: {:?}", arr2);

    let mut index = String::new();

    io::stdin().read_line(&mut index).expect("Failed to read input");

    let index: usize = index.trim().parse().expect("Failed to parse input");
    println!("The value of index is: {}", index);
    println!("The value of arr2 at index {} is: {}", index, arr2[index]); // This will panic if index is out of bounds
}
