struct Rectangle {
    width: u8,
    height: u8,
}

impl Rectangle {
    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

fn main() {
    println!("Hello, world!");
}

fn get_two() -> i32 {
    2
}

#[cfg(test)]
mod dcode_tests {
    #[test]
    fn test_basic() {
        assert!(1 == 1);
    }

    #[test]
    #[should_panic]
    fn test_panic() {
        panic!("Oh no");
    }

    #[test]
    fn test_equals() {
        assert_eq!(2, 1 + 1);
    }

    #[test]
    fn test_not_equals() {
        assert_ne!(2, 1 + 2);
    }

    #[test]
    #[ignore]
    fn test_ignore() {
        assert_eq!(5, 3 + 2);
    }

    #[test]
    fn test_outer_scope() {
        assert_eq!(super::get_two(), 2);
    }

    #[test]
    fn test_structs() {
        let r = super::Rectangle {
            width: 50,
            height: 25,
        };

        assert!(!r.is_square());

        let s = super::Rectangle {
            width: 100,
            height: 100,
        };

        assert!(s.is_square());
    }
}
