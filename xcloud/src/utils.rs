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

    /// Ensures that the given path exists by creating any missing directories.
    ///
    /// # Arguments
    ///
    /// * `path` - A PathBuf that holds the path to check.
    ///
    /// # Returns
    ///
    /// * `Result<()>` - A result indicating success or failure.
    pub fn ensure_path_exists(path: std::path::PathBuf) -> std::io::Result<()> {
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent).expect("Failed to create directories");
            }
        }
        Ok(())
    }

    /// Constructs a path from the given components.
    ///
    /// # Arguments
    ///
    /// * `components` - A slice of string slices representing the components of the path.
    ///
    /// # Returns
    ///
    /// * `PathBuf` - The constructed path.
    pub fn get_path(components: &[&str]) -> std::path::PathBuf {
        let db_path = dirs::data_dir().expect("Could not determine data directory");
        components.iter().fold(db_path, |mut path, &component| {
            path.push(component);
            path
        })
    }
}
