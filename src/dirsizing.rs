use crate::{options::DuxOptions, results::DuxResults};
use std::fs::read_dir;
use std::fs::Metadata;

pub fn du_get_size(file: String, root: Option<String>, res: &mut DuxResults, opts: &DuxOptions) {
    let md: Metadata;
    let statopt = std::fs::metadata(&file);
    match statopt {
        Ok(data) => md = data,
        Err(e) => {
            println!("{}", e);
            return;
        }
    }

    if md.is_dir() {
        let dir: std::fs::ReadDir;
        let diropt = read_dir(&file);
        match diropt {
            Ok(d) => dir = d,
            Err(e) => {
                println!("{}", e);
                return;
            }
        }

        for ent in dir {
            // TODO : x -plat via filepath.Join thingie
            let entry = ent.unwrap();
            let name = entry.file_name();
            let name = name.to_str().unwrap();

            let path = format!("{}/{}", &file, &name);
            let subroot: Option<String>;
            match root {
                Some(ref r) => subroot = Some(r.to_string()),
                None => {
                    subroot = Some(file.clone());
                }
            }

            du_get_size(path, subroot, res, opts)
        }

        return;
    }

    res.add(file, md.len(), opts, root)
}
