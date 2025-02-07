#[derive(Debug)] // add to `impl Debug for Rectangle`

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn set_width(&mut self, width: u32) {
        self.width = width;
    }
    
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
    
    fn max(self, other: Rectangle) -> Rectangle {
        Rectangle {
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }
}


fn main() {
    let rect1 = Rectangle {
    width: 30,
    height: 50,
    };
    let mut rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    let sq1 = Rectangle::square(10);
    
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
    println!(
        "Using a method: the area of the rectangle is {} square pixels.",
        rect1.area()
    );
    // the specifier :? inside the curly brackets tells println! 
    // we want to use Debug output format
    println!("rect1 is {rect1:?}");

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    
    println!("what's in the square is {sq1:?}");
    
    let a1 = rect3.area();
    let a2 = Rectangle::area(&rect3);
    assert!(a1 == a2);
    // rect.area() and Rectangle::area(&rect) are equivalent
    rect2.set_width(7);
    Rectangle::set_width(&mut rect2, 8);
    
    let rect4 = rect3.max(rect2);
    
    println!("which was the max? {rect4:?}");
    
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
