// Code that lists the dir, traversing it like a DAG.
pub mod walk {
    use std::{fs, io, path::Path};
    pub fn list_dir(path: String) -> Result<Vec<String>, (io::Error, Vec<String>)> {
        let path_struct = Path::new(&path);
        if path_struct.is_dir() {
            list_dirs_dfs(path_struct)
        } else {
            return Ok(vec![path]);
        }
    }

    fn list_dirs_dfs(dir: &Path) -> Result<Vec<String>, (io::Error, Vec<String>)> {
        let mut res: Vec<String> = Vec::<String>::new();
        let entries = match fs::read_dir(dir){
            Ok(read_dir) => read_dir,
            Err(err) => { println!("failed to read dir {:?}", dir); return Err((err, res));},
        };
        for entry in entries{
            let path_buf = match entry {
                Ok(ent) => ent,
                Err(err) => { return Err((err, res)); },
            }.path();
            let path = path_buf.as_path();
            if path.is_dir() {
                match list_dirs_dfs(path) {
                    Ok(v) => res.extend(v),
                    Err((err, v)) => {
                        res.extend(v);
                        return Err((err, res));
                    },
                }
            } else {
                // println!("Adding {:?}", path);
                res.push(path.as_os_str().to_str().expect("String parse error").to_string());
            }
        }

        Ok(res)
    }
}
