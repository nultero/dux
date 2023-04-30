use crate::{options::DuxOptions, results::DuxSingle};

const KB: u64 = 1000 - 1;
const MB: u64 = KB * 1000;
const GB: u64 = MB * 1000;
const TB: u64 = GB * 1000;
const PB: u64 = TB * 1000;

const KBF: f64 = 1000.0 - 1.0;
const MBF: f64 = KBF * 1000.0;
const GBF: f64 = MBF * 1000.0;
const TBF: f64 = GBF * 1000.0;
// const PBF: f64 = TBF * 1000.0;

const OFFSET: usize = 3;

type FnPtr = Box<dyn Fn(String, u64) -> String>;

fn gray_bytes(s: String, _: u64) -> String {
    return format!("\x1b[38;2;80;80;80m{}\x1b[0m", s);
}

fn lukewarm_kbytes(s: String, sz: u64) -> String {
    let color_percent = (sz as f64) / MBF;
    let mut reduxer = 60.0 * color_percent;
    let red = 80 + (reduxer as i32);
    reduxer = 80.0 + (0.65 * reduxer);
    let green = reduxer as i32;
    let blue = (80.0 - (80.0 * color_percent)) as i32;
    return format!("\x1b[38;2;{};{};{}m{}\x1b[0m", red, green, blue, s);
}

fn mild_meg_bytes(s: String, sz: u64) -> String {
    let color_percent = (sz as f64) / GBF;
    let mut reduxer = 115.0 * color_percent;
    let red = 140 + (reduxer as i32);
    reduxer = 110.5 + (0.65 * reduxer);
    let green = reduxer as i32;
    return format!("\x1b[38;2;{};{};1m{}\x1b[0m", red, green, s);
}

fn hot_gigs(s: String, sz: u64) -> String {
    let color_percent = (sz as f64) / TBF;
    let reduxer = 1.0 - color_percent;
    let green = (160.0 * reduxer) as i32;
    return format!("\x1b[38;2;255;{};1m{}\x1b[0m", green, s);
}

fn get_fmt_str(ds: &mut DuxSingle, color: bool, fnptrs: &Vec<FnPtr>) -> (String, usize) {
    let mut nchars: usize = 0;
    let mut f: Option<&FnPtr> = None;
    let mut s: String;
    match ds.size {
        ..=KB => {
            s = format!("{:.0}B", ds.size);
            nchars = s.len();
            f = Some(&fnptrs[0]);
        }
        ..=MB => {
            s = format!("{}KB", ds.size / KB);
            nchars = s.len();
            f = Some(&fnptrs[1]);
        }
        ..=GB => {
            s = format!("{}MB", ds.size / MB);
            nchars = s.len();
            f = Some(&fnptrs[2]);
        }
        ..=TB => {
            s = format!("{:.1}GB", (ds.size as f64) / GBF);
            nchars = s.len();
            f = Some(&fnptrs[3]);
        }
        ..=PB => {
            s = format!("{:.1}TB", (ds.size as f64) / TBF);
            nchars = s.len();
            f = Some(&fnptrs[0]);
        }
        _ => {
            return ("".to_string(), 0);
        }
    }

    if color {
        if let Some(do_fn) = f {
            s = do_fn(s, ds.size);
        }
    }

    ds.size = nchars as u64;
    return (s, nchars);
}

pub fn display_results(files: &mut Vec<DuxSingle>, opts: &DuxOptions) {
    // the optionally-colorful size strs on the left
    let mut substrs: Vec<String> = Vec::with_capacity(files.len());

    let mut fnptrs: Vec<FnPtr> = vec![];
    fnptrs.push(Box::new(gray_bytes));
    fnptrs.push(Box::new(lukewarm_kbytes));
    fnptrs.push(Box::new(mild_meg_bytes));
    fnptrs.push(Box::new(hot_gigs));

    let mut maxlen: usize = 0;

    for f in files.into_iter() {
        let (s, nchars) = get_fmt_str(f, opts.color, &fnptrs);
        if nchars > maxlen {
            maxlen = nchars;
        }
        substrs.push(s);
    }

    maxlen += OFFSET;

    let mut idx: usize = 0;
    loop {
        if idx == files.len() {
            break;
        }

        let f = &files[idx];

        let diff = maxlen - (f.size as usize);
        println!(" {}{}{}", substrs[idx], " ".repeat(diff), f.name);
        idx += 1;
    }

    std::mem::forget(substrs);
}
