use std::{fs, io};
fn main() -> io::Result<()> {
    for entry in fs::read_dir(".")? {
        let dir = entry?;
        println!("{}", dir.path().to_string_lossy());
    }
    Ok(())
}
