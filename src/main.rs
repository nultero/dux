mod dir_fns;

use std::collections::HashMap;
use dir_fns::*;

const KB: u64 = 1000;
const MB: u64 = 1000 * KB;
const GB: u64 = 1000 * MB;

fn wd_size() {
    let mut hm: HashMap<u64, String> = HashMap::with_capacity(40);
    let wd = std::env::current_dir()
                    .expect("couldn't get current dir");

    get_dir_sz_m(wd, &mut hm);

    let mut sizes: Vec<&u64> = hm.keys().collect();
    sizes.sort();

    let mut sz_strs: Vec<String>    = vec![String::new(); sizes.len()];
    let mut file_names: Vec<String> = vec![String::new(); sizes.len()];
    let mut mx_len = 0;

    for (i, sz) in sizes.iter().enumerate() {
        let f = hm.get(sz);
        if let Some(fname) = f {
            let fsz = fmtsz(sz);
            if fsz.len() > mx_len {
                mx_len = fsz.len();
            }

            sz_strs[i] = fsz;
            file_names[i] = fname.as_str().to_owned();
        }
    }

    mx_len += 3;

    for (i, sz) in sz_strs.iter().enumerate() {
        println!(
            " {}:{}{}",
            sz, 
            " ".repeat(mx_len - sz.len()),
            file_names[i],
        );
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
    match bytes {
        GB..  => {   return format!("{} GB", bytes/GB);  }
        MB..  => {   return format!("{} MB", bytes/MB);  }
        KB..  => {   return format!("{} KB", bytes/KB);  }
        _     => {   return format!("{} B", bytes);  }
    }
}
