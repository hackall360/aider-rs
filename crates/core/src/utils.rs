use std::path::{Path, PathBuf};

const IMAGE_EXTENSIONS: [&str; 8] = [
    ".png", ".jpg", ".jpeg", ".gif", ".bmp", ".tiff", ".webp", ".pdf",
];

/// Check if the provided file name has an image extension.
pub fn is_image_file<P: AsRef<Path>>(file_name: P) -> bool {
    let file_name = file_name.as_ref().to_string_lossy().to_lowercase();
    IMAGE_EXTENSIONS.iter().any(|ext| file_name.ends_with(ext))
}

/// Return a canonicalized absolute path as a String.
pub fn safe_abs_path<P: AsRef<Path>>(path: P) -> String {
    let path: PathBuf = path.as_ref().to_path_buf();
    match path.canonicalize() {
        Ok(p) => p.to_string_lossy().into_owned(),
        Err(_) => path.to_string_lossy().into_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_is_image_file() {
        assert!(is_image_file("test.png"));
        assert!(!is_image_file("test.txt"));
    }

    #[test]
    fn test_safe_abs_path() {
        let tmp = tempfile::tempdir().unwrap();
        let file = tmp.path().join("a.txt");
        fs::write(&file, "hi").unwrap();
        let abs = safe_abs_path(&file);
        assert!(abs.ends_with("a.txt"));
    }
}
