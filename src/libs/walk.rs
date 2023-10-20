// Code that lists the dir, traversing it like a DAG.
pub mod walk {
    use std::{fs, io};
    pub fn list_dir(path: String) -> Result<Vec<String>, io::Error> {
        Ok(fs::read_dir(path)?.map(|read_dir| read_dir.map(|entry| -> String {entry.path()
            .as_os_str().to_str().expect("String Parse Error").to_string()}))
           .collect::<Result<Vec<_>, io::Error>>()?)
    }
}
