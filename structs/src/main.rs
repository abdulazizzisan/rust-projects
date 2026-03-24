use derive_builder::Builder;

#[derive(Debug, Builder)]
struct User{
    name: String,
    username: String,
    email: String,
    password: String,
    active: bool
}

fn main() {

    // create instance
    let user = User{
        name: "Md Abdul Aziz Zisan".to_string(),
        username: "zisan".to_string(),
        email: "hello@zisan.dev".to_string(),
        password: "zisanzisan".to_string(),
        active: false,
    };

    println!("User created using regular method: \n {:#?}", user); // print user formatted.
    println!();

    // Mutate Struct Instance
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

    // create instance with builder
    let built_user = UserBuilder::default()
        .username("zisan".to_string())
        .email("email@email.com".to_string())
        .password("password".to_string())
        .active(true)
        .name("Md Abdul Aziz Zisan".to_string())
        .build()
        .expect("Error building user");

    println!("User built using builder: {:#?}", built_user);

    // updating user with struct update syntax

    let user = User{
        name: "zisan bro".to_string(),
        ..built_user
    };

    println!("User instance with update struct syntax: {:#?}", user);
    println!("active status of old user: {}", built_user.active); // works fine because has copy trait
    // println!("username of old user: {}", built_user.username); // panics as username moved to user
    println!("name of old user: {}", built_user.name); // works fine because not moved.

}
