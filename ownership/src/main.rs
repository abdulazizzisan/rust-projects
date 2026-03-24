fn main() {
    let str = String::from("Hello, world!");
    first_word(&str);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i
        }
        println!("item: {item}\nindex: {i}\n\n");
    }
    s.len()
}