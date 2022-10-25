#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn new(w: u32, h: u32) -> Rectangle {
        Rectangle {
            width: w,
            height: h,
        }
    }

    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    pub fn widden(&mut self) {
        self.width *= 2;
        self.height *= 2;
    }

    pub fn is_square(&self) -> bool {
        self.width == self.height
    }
}

pub fn play_with_rectangle() {
    let mut rect = Rectangle::new(10, 10);
    assert!(rect.is_square());

    rect.widden();

    println!("Area: {}", rect.area());
}

#[cfg(test)]
mod tests {
    use crate::traits::structs::Rectangle;

    #[test]
    fn square() {
        let rect = Rectangle::new(10, 10);
        assert_eq!(rect.is_square(), true);
    }

    #[test]
    fn not_square() {
        let rect = Rectangle::new(10, 15);
        assert_eq!(rect.is_square(), false);
    }

    #[test]
    fn widden() {
        let mut rect = Rectangle::new(10, 15);
        rect.widden();
        assert_eq!(rect.width, 20);
        assert_eq!(rect.height, 30);
    }

    #[test]
    fn area() {
        let rect = Rectangle::new(10, 15);
        assert_eq!(rect.area(), 150);
    }
}
