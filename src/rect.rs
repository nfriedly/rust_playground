pub fn rect_math() {
    let mut rect1 = Rectangle {
        length: 30,
        width: 50
    };

    println!(
        "The area of the rectangle is {}",
        rect1.area()
    );
    rect1.widen();
    println!(
        "The area of the rectangle is {}",
        rect1.area()
    );
    println!("rect1 is {:#?}", rect1);

    let sq = Square {
        width: 20
    };

    println!("sq width: {:?}", sq);

    let sq2 = sq.to_rectangle(); // comsumes the Square, returns a Rectangle

    //println!("sq width: {}", sq.width);// won't work anymore

    println!(
        "The area of the square (rectangle) is {}",
        sq2.area()
    );
    println!("sq is {:#?}", sq2);
}


#[derive(Debug)]
struct Square {
    width: u32
}

impl Square {
    fn to_rectangle(self) -> Rectangle {
        Rectangle {
            length: self.width,
            width: self.width
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32
}


impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }
    fn widen(&mut self) {
        self.width = self.width + 10;
    }
}
