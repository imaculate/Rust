#[derive(Debug)]
struct Rectangle
{
    height : i32,
    width: i32
}

impl Rectangle
{
    fn can_hold(&self, other: &Rectangle) -> bool
    {
        self.height > other.height && self.width > other.width
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_larger_can_hold_smaller() {
        let larger = Rectangle
        {
            height : 8,
            width: 5
        };

        let smaller = Rectangle
        {
            height: 7,
            width: 1
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_can_hold_larger()
    {
        let smaller = Rectangle
        {
            height : 5,
            width: 1
        };

        let larger = Rectangle
        {
            height: 8,
            width: 7
        };
        assert!(!smaller.can_hold(&larger));
    }

}
