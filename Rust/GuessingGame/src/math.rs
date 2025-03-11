// src/math.rs
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)] // Compile this block only when running tests
mod tests {
    use super::*; // Import everything from the parent module (`math`)

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}
