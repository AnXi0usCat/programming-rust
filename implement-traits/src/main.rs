#[allow(dead_code)]

pub struct Canvas;

pub struct Broom {
    x: i32,
    y: i32,
    height: i32
}

impl Canvas {

    fn write_at(x: i32, y: i32, c: char) {
        println!("{}, {}, {}", x ,y ,c);
    }
}

/// A trait for characters, items and scenery
/// anything that is visible
trait Visible {
    /// render this object on a given canvas
    fn draw(&self, canvas: &Canvas);

    /// return true is clicking on x and y returns
    /// the object
    fn hit_test(&self, x: i32, y: i32) -> Bool;
}

impl Broom {
    /// helper function used by Broom::draw()
    fn broomstick_range(&self) -> Range<i32> {
        self.y - self.height - 1 .. self.y
    }
}

impl Visible for Broom {

    fn draw(&self, canvas: &Canvas) {
        for y in self.broomstick_range() {
            canvas.write_at(self.x, y, '|')
        }
        canvas.write_at(self.x, self.y, '|')
    }

    fn hit_test(&self, x: i32, y: i32) -> Bool {
        self.x == x
        && self.y - self.height - 1 <= y
        && y <= self.y
    }
}


fn main() {
    println!("Hello, world!");
}
