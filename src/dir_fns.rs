use std::{
    path::PathBuf, 
    fs::DirEntry
};

pub fn get_file_sz(de: DirEntry) -> u64 {
    let mut size: u64 = 0;
    let metadata = de
            .metadata()
            .expect("trouble getting metadata of file");

    if metadata.is_dir() {
        size += get_dir_sz(de.path());

    } else {
        size += metadata.len();
    }
    return size;
}

fn get_dir_sz(d: PathBuf) -> u64 {
    let wd = std::fs::read_dir(d.as_path())
                        .expect("couldn't read dir");

    let mut size_sum: u64 = 0;

    for dirent in wd {
        let de = dirent.expect("trouble getting direntry of file");
        size_sum += get_file_sz(de);
    }

    return size_sum;
}