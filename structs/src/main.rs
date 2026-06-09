
#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }
    fn can_hold(&self, rect: &Rectangle) -> bool {
        rect.area() < self.area()
    }
    fn square(size: u32) -> Self {
        Self {
            length: size,
            width: size,
        }
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

fn main() {
    let scale = 2;

    let rect = Rectangle {
        length: dbg!(30 * scale), 
        width:50
    };

    println!("rect is: {rect:#?}");

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect)
    );
    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );

    dbg!(&rect);

    let rect1 = Rectangle {
        length: 30,
        width: 50,
    };
    let rect2 = Rectangle {
        length: 10,
        width: 40,
    };
    let rect3 = Rectangle {
        length: 60,
        width: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square = Rectangle::square(42);

    println!("{square:?}")

}

fn area(shape: &Rectangle) -> u32 {
    shape.length * shape.width
}