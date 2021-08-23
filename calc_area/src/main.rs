//! with a tupple

// fn main() {
//     let rect1 = (30, 50); 

//     println!("The area of the rectangle is {} square pisxels.", area(rect1)
//   );
// }

// fn area(dimentions: (u32, u32)) -> u32 {
//   dimentions.0 * dimentions.1
// }

//! With a struct 
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 60,
    };
    
    let rect3 = Rectangle {
        width: 20,
        height: 40,
    };
    
    println!("The area of the rectangle 1 is {} square pisxels.", rect1.area()
  );
  
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!("{:?}", rect1); // Debug mode
    println!("{:#?}", rect2); // Debug mode fancy way
    println!("{:#?}", rect3); 

    // Associated function
    let square1 = Rectangle::square(3);
    println!("{:#?}", square1);

}

#[derive(Debug)] // Debug mode to be able to print complex data type
struct Rectangle {
    width: u32,
    height: u32,
}


impl Rectangle {
    // Methods
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
     // Associated functions
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}



