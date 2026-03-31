#[derive(Debug)]
struct Rectangle {
    height: i32,
    width: i32,
}

impl Rectangle {
    fn area(&self) -> i32 {
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        // if dbg!(self.area()) >= dbg!(rect.area()) {
        //     true
        // } else {
        //     false
        // }
        if rect.height <= self.height && rect.width <= self.width {
            true
        } else {
            false
        }
    }
}

fn main() {
    let rect = Rectangle {
        height: 6,
        width: 2,
    };

    println!("The area of rectange with struct: {}", area_struct(&rect));
    println!(
        "The area of the rectangle is {} square pixels",
        area(rect.width, rect.height)
    );

    println!("{rect:?}"); // unformatted struct print {needs derive Debug}
    println!("{rect:#?}"); // formatted struct print {needs derive Debug}
    println!("Area using method {}", rect.area());

    println!("==============Can Hold Program=================");
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    let rect4 = Rectangle {
        width: 5,
        height: 51,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect4));
}
fn area(width: i32, height: i32) -> i32 {
    width * height
}

fn area_struct(rectangle: &Rectangle) -> i32 {
    rectangle.width * rectangle.height
}
