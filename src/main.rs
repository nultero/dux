mod dir_fns;
mod display;

use std::collections::HashMap;
use std::fs::read_dir;
use dir_fns::*;
use display::*;

fn cwd_size(map: &mut HashMap<u64, String>) {
    let wpath = std::env::current_dir()
                    .expect("couldn't get current dir path");
    
    let cwd = read_dir(wpath).expect("couldn't get current dir");

    for e in cwd {
        let entry = e.expect("problem converting filename to direntry");
        let f = &entry.file_name();
        let sz = get_file_sz(entry);
        let fopt = f.to_str();
        if let Some(name) = fopt {
            let opt = map.insert(sz, name.to_owned());
            match opt {
            Some(_) => {},
            None => {}
            }
        }
    }
}

fn main() {
    let mut map: HashMap<u64, String> = HashMap::with_capacity(40);
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.len() == 0 {
        cwd_size(&mut map);
    }

    display(map);
}