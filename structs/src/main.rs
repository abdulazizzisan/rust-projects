#[derive(Debug)]
struct User{
    name: String,
    username: String,
    email: String,
    password: String,
    active: bool
}

fn main() {

    let user = User{
        name: "Md Abdul Aziz Zisan".to_string(),
        username: "zisan".to_string(),
        email: "hello@zisan.dev".to_string(),
        password: "zisanzisan".to_string(),
        active: false,
    };

    println!("User created using regular method: \n {:#?}", user); // print user formatted.
    println!();

    let mut mutable_user = User{
        name: "Md Abdul Aziz Zisan".to_string(),
        username: "zisan".to_string(),
        email: "hello@zisan.dev".to_string(),
        password: "zisanzisan".to_string(),
        active: false,
    };

    println!("Mutable user before mutating: \n {:#?}", mutable_user);

    mutable_user.username = "zisan_mutated".to_string();

    println!("Mutable user after mutating: \n {:#?}", mutable_user);
}
