use std::collections::HashMap;

use crate::options::DuxOptions;

pub struct DuxResults {
    map: HashMap<String, u64>,
}

#[derive(PartialEq, Eq)]
pub struct DuxSingle<'a> {
    pub name: &'a String,
    pub size: u64,
}

impl Ord for DuxSingle<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.size.cmp(&other.size);
    }
}

impl PartialOrd for DuxSingle<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl DuxResults {
    pub fn add(&mut self, file: String, size: u64, opts: &DuxOptions, root: Option<String>) {
        if size < 1 {
            return;
        }

        if opts.recurse {
            self.map.insert(file, size);
            return;
        }

        let r: String;
        match root {
            Some(s) => r = s,
            None => {
                self.map.insert(file, size);
                return;
            }
        }

        let valopt = self.map.get(&r);
        match valopt {
            Some(val) => {
                self.map.insert(r, val + size);
            }
            None => {
                self.map.insert(r, size);
            }
        }
    }

    pub fn get_sorted(&self) -> Vec<DuxSingle> {
        let n = self.map.len();
        let mut files = Vec::with_capacity(n);
        for (k, v) in self.map.iter() {
            let sing = DuxSingle { name: k, size: *v };
            files.push(sing);
        }
        files.sort();
        return files;
    }

    // pub fn dump(&self) {
    //     for (k, v) in self.map.iter() {
    //         println!("{} | {}", k, v);
    //     }
    // }
}

pub fn new_dux_results() -> DuxResults {
    return DuxResults {
        map: HashMap::new(),
    };
}
