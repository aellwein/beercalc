use std::{fs::OpenOptions, io::Write, path::PathBuf, process};

fn main() -> Result<(), std::io::Error> {
    let output: Vec<u8> = process::Command::new("git")
        .args(["describe", "--tags", "--always", "HEAD"])
        .output()?
        .stdout;

    let git_sha = String::from_utf8_lossy(&output).trim().to_string();
    let path = PathBuf::from("assets/commit_sha.txt");
    OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)?
        .write_all(git_sha.as_bytes())?;

    Ok(())
}