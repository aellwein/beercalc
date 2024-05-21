use anyhow::Result;
use vergen::EmitBuilder;

fn main() -> Result<()> {
    EmitBuilder::builder().git_sha(true).emit()?;

    std::process::Command::new("npx")
        .args([
            "tailwindcss",
            "-i",
            "input.css",
            "-o",
            "public/assets/css/tailwind.css",
            "--minify",
        ])
        .spawn()?;
    Ok(())
}
