// TDD Tests for Rust Demo Tauri Application

#[cfg(test)]
mod tests {
    use rust_demo::*;

    #[test]
    fn test_greet_with_normal_name() {
        let result = greet("World");
        assert_eq!(result, "Hello, World!");
    }

    #[test]
    fn test_greet_with_empty_string() {
        let result = greet("");
        assert_eq!(result, "Hello, !");
    }

    #[test]
    fn test_greet_with_special_characters() {
        let result = greet("Rust & Tauri");
        assert_eq!(result, "Hello, Rust & Tauri!");
    }

    #[test]
    fn test_greet_with_unicode() {
        let result = greet("世界");
        assert_eq!(result, "Hello, 世界!");
    }
}
