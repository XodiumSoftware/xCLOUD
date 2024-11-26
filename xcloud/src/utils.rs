/// Utility functions for the xcloud crate.
pub struct Utils;

/// Implementation of the `Utils` struct.
impl Utils {
    /// Sanitizes the given string by removing all non-alphanumeric characters.
    ///
    /// # Arguments
    ///
    /// * `str` - A string slice to sanitize.
    ///
    /// # Returns
    ///
    /// * `String` - The sanitized string.
    pub fn sanitize(str: &str) -> String {
        str.chars()
            .filter(|c| c.is_alphanumeric() || *c == '_')
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize() {
        let input = "Hello, World!";
        let expected = "HelloWorld";
        assert_eq!(Utils::sanitize(input), expected);
    }
}
