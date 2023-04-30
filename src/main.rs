use dirsizing::du_get_size;
use display::display_results;
use options::get_options;
use results::new_dux_results;

mod dirsizing;
mod display;
mod options;
mod results;

fn main() {
    let (duxopts, mut args) = get_options();
    let mut res = new_dux_results();

    if args.len() == 0 {
        for f in std::fs::read_dir(".").unwrap() {
            let entry = f.unwrap();
            let name = entry.file_name();
            let name = name.to_str().unwrap();

            if name.starts_with(".") && !duxopts.all {
                continue;
            }

            args.push(name.to_string());
        }
    }

    for arg in args {
        du_get_size(arg, None, &mut res, &duxopts);
    }

    let mut files = res.get_sorted();
    display_results(&mut files, &duxopts);

    std::mem::forget(files);
    std::mem::forget(res);
}
