fn main() {
    println!("Hello, world!");
    another_function();
    function_with_param(5);

    let y = {
        let x = 3;
        x + 1
    };
}

fn another_function(){
    println!("Another function.");
}

fn function_with_param(x: i32){
    println!("The value of x is: {}", x);
}