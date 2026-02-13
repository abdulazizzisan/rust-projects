fn main() {
    let number = 9;
    if number < 5 {
        println!("condition was true");
    } else if number == 5 {
        println!("number is five");
    } else {
        println!("condition was false")
    }

    let condition = false;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    let a: isize = isize::MAX;

    let b: i32 = a as i32;

    println!("The value of a is: {a}");

    println!("The value of b is: {b}");
    loop {
        println!("again!");
        break;
    }

    let mut count = 0;
    let result = loop {
        count += 1;

        if count == 10 {
            break count;
        };
    };
    println!("The result is {result}");
    print_array();
    println!();
    for_each();
}

fn print_array(){
    let mut a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < a.len() {
        println!("the value is: {}", a[index]);
        a[index] = 5;
        index+=1;
    }

    for i in a {
        println!("{i}")
    }
}

fn for_each(){
    let a = [10, 20, 30, 40, 50];

    for mut item in a{

        if item == 30 { item = 40 }
        println!("the value is: {}", item);
    }

    for i in a {
        println!("{i}");
    }
}
