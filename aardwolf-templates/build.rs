use ructe::{Result, Ructe};

fn main() -> Result<()> {
    Ructe::from_env()?.compile_templates("templates")?;
    println!("cargo:rerun-if-changed=templates");
    Ok(())
}