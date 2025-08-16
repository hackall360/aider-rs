use anyhow::Result;
use ignore::WalkBuilder;

fn main() -> Result<()> {
    let root = std::env::current_dir()?;
    for entry in WalkBuilder::new(&root)
        .standard_filters(true)
        .build()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().map(|ft| ft.is_file()).unwrap_or(false))
    {
        let path = entry.into_path();
        if let Ok(rel) = path.strip_prefix(&root) {
            println!("{}", rel.display());
        } else {
            println!("{}", path.display());
        }
    }
    Ok(())
}
