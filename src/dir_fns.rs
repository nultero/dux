use std::{
    collections::HashMap, 
    path::PathBuf
};

pub fn get_dir_sz(d: PathBuf) -> u64 {
    let wd = std::fs::read_dir(d.as_path())
                        .expect("couldn't read dir");

    let mut size_sum: u64 = 0;

    for dirent in wd {
        let de = dirent.expect("trouble getting direntry of file");
        let metadata = de
                                .metadata()
                                .expect("trouble getting metadata of file");
    
        if metadata.is_dir() {
            size_sum += get_dir_sz(de.path());

        } else {
            size_sum += metadata.len();
        }
    }

    return size_sum;
}

pub fn get_dir_sz_m(wd: PathBuf, hm: &mut HashMap<u64, String>) {
    let cwd = std::fs::read_dir(wd.as_path())
                        .expect("couldn't read current dir");

    for dirent in cwd {
        let de = dirent.expect("trouble getting direntry of file");
        let metadata = de
                                .metadata()
                                .expect("trouble getting metadata of file");

        let mut dir_sz: u64 = 0;
        if metadata.is_dir() {
            dir_sz = get_dir_sz(de.path());
        }

        let name_opt = de.file_name();
        if let Some(name) = name_opt.to_str() {
            let mut res = hm.insert(metadata.len() + dir_sz, name.to_owned());
            let mut i = 1;
            // if another file has the exact same size,
            // they will collide in the map, so just increment
            // by a tiny bit and it's fine
            while let Some(_) = res {
                res = hm.insert(metadata.len() + i + dir_sz, name.to_owned());
                i += 1;
             };
        } else { 
            continue; 
        }
    }
}