//! Test
//!
//! ```
//! use workflow_cargo_test;
//! assert_eq!(workflow_cargo_test::doctest(), 42);
//! ```

pub fn test() -> usize {
    42
}

pub fn doctest() -> usize {
    42
}

pub fn x() -> usize {
    42
}

pub fn y() -> usize {
    42
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert_eq!(super::test(), 42);
    }

    #[cfg(feature = "x")]
    #[test]
    fn x() {
        assert_eq!(super::x(), 42);
    }

    #[cfg(feature = "y")]
    #[test]
    fn y() {
        assert_eq!(super::y(), 42);
    }
}
