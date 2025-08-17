use super::CoderError;

/// Perform a simple search and replace on the provided content.
/// Returns an error if the search string is not found.
pub fn search_replace(content: &str, search: &str, replace: &str) -> Result<String, CoderError> {
    if !content.contains(search) {
        return Err(CoderError::NotFound);
    }
    Ok(content.replace(search, replace))
}
