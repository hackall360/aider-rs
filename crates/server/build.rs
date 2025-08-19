fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .compile(&["../../proto/aider.proto"], &["../../proto"])?;
    println!("cargo:rerun-if-changed=../../proto/aider.proto");
    Ok(())
}
