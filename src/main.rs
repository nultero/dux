use std::collections::HashMap;

const KB: u64 = 1000;
const MB: u64 = 1000 * KB;
const GB: u64 = 1000 * MB;

fn wd_size() {
    let mut hm: HashMap<u64, String> = HashMap::with_capacity(40);

    let wd = std::env::current_dir()
                        .expect("couldn't get current dir");

    let cwd = std::fs::read_dir(wd.as_path())
                        .expect("couldn't read current dir");

    for dirent in cwd {
        let de = dirent.expect("trouble getting direntry of file");
        let metadata = de
                                .metadata()
                                .expect("trouble getting metadata of file");

        let name_opt = de.file_name();
        if let Some(name) = name_opt.to_str() {
            let mut res = hm.insert(metadata.len(), name.to_owned());
            let mut i = 1;
            while let Some(_) = res {
                res = hm.insert(metadata.len()+i, name.to_owned());
                i += 1;
             };
        } else { 
            continue; 
        }
    }

    let mut sizes: Vec<&u64> = hm.keys().collect();
    sizes.sort();

    for sz in sizes {
        let f = hm.get(sz);
        if let Some(fname) = f {
            let fsz = fmtsz(sz);
            println!("{}: {:?}", fsz, fname);
        }
    }


}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.len() == 0 {
        wd_size();
        return;
    }

    for arg in args {
        println!("{}", arg);
    }
}


fn fmtsz(bytes: &u64) -> String {
    let mut s: String = String::with_capacity(10);
    match bytes {
        GB..  => {   s = format!("{} GB", bytes/GB);  }
        MB..  => {   s = format!("{} MB", bytes/MB);  }
        KB..  => {   s = format!("{} KB", bytes/KB);  }
        _     => {   s = format!("{} B", bytes);  }
    }
    return s;
}
