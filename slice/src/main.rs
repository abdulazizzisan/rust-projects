fn main() {
    hello_world_slice();
    println!("first word: {}", first_word(&String::from("Hello World")));
    println!("second word: {}", second_word(&String::from("Hello World")));
    let mut s = String::from("Hello World");
    let first_word = first_word(&s);
    s.clear();


    println!("first word: {}", first_word);
}

fn hello_world_slice(){
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..s.len()];

    println!("Hello: {hello}");
    println!("world: {world}");
}

fn first_word(s: &str) -> &str{
    let s_bytes = s.as_bytes();
    for (i, &item) in s_bytes.iter().enumerate(){
        if item == b' '{
            return &s[0..i];
        }
    }
    &s[..]
}

fn second_word(s: &str) -> &str{
    let s_bytes = s.as_bytes();
    for(mut i, &item) in s_bytes.iter().enumerate(){
        if item == b' '{
            i+=1;
            return first_word(&s[i..])
        }
    }
    &s
}
