use std::collections::HashMap;

const B_: u64 = 0;
const KB: u64 = 1000;
const MB: u64 = 1000 * KB;
const GB: u64 = 1000 * MB;
const BGB: u64 = 100 * GB;

const FKB: f32 = KB as f32;
const FMB: f32 = MB as f32;
const FGB: f32 = GB as f32;
const FBGB: f32 = BGB as f32;

const BS: &str = "B ";
const KBS: &str = "KB";
const MBS: &str = "MB";
const GBS: &str = "GB";

fn get_color_str(bytes: &u64, type_col: u64, nxt_threshold: f32) -> String {
    let fbytes = bytes.clone() as f32;
    let mut color_percent = fbytes / nxt_threshold;
    if color_percent > 1.0 {
        color_percent = 1.0;
    }

    if type_col == GB { // deep orange -> red
        let g_redux = 1.0 - color_percent;
        let g_colr: i32 = (160.0 * g_redux) as i32; // lossy, don't care
        return format!("\x1b[38;2;255;{};1m", g_colr);
        

    } else if type_col == MB { // light orange -> deep orange
        let r_redux = 115.0 * color_percent;
        let r_colr = 140 + (r_redux as i32);
        
        let g_redux: f32 = 110.5 + (0.65 * r_redux);
        let g_colr: i32 = g_redux as i32;
        
        return format!("\x1b[38;2;{};{};1m", r_colr, g_colr);


    } else if type_col == KB { // gray -> light orange

        let r_redux = 60.0 * color_percent;
        let r_colr = 80 + (r_redux as i32);
        
        let g_redux: f32 = 80.0 + (0.65 * r_redux);
        let g_colr: i32 = g_redux as i32; // lossy, don't care

        let b_plus: f32 = 80.0 - (80.0 * color_percent);
        let b_colr = b_plus as i32;
        
        return format!("\x1b[38;2;{};{};{}m", r_colr, g_colr, b_colr);
        
    } else { // bytes are just a static gray
        return format!("\x1b[38;2;80;80;80m");
    }
}

fn getsz(bytes: &u64) -> (String, String, String) {
    match bytes {
        GB..  => {
            return (
                (bytes/GB).to_string(), 
                GBS.to_owned(),
                get_color_str(bytes, GB, FBGB)
            );
        }
        MB..  => { 
            return ( 
                (bytes/MB).to_string(), 
                MBS.to_owned(),
                get_color_str(bytes, MB, FGB)
            );
        }
        KB..  => { 
            return ( 
                (bytes/KB).to_string(), 
                KBS.to_owned(),
                get_color_str(bytes, KB, FMB)
            );
        }
        _  => {
            return ( 
                bytes.to_string(), 
                BS.to_owned(),
                get_color_str(bytes, B_, FKB)
            );
        }
    }
}

fn fmtsz(sz: &[String; 3]) -> String {
    let s = &sz[1];
    if s == &BS {
        return format!("{}{}{}\x1b[0m", sz[2], sz[0], sz[1]);

    } else if s == &KBS {
        return format!("{}{}{}\x1b[0m", sz[2], sz[0], sz[1]);

    } else if s == &MBS {
        // return format!("\x1b[38;2;230;140;1m{}{}\x1b[0m", sz[0], sz[1]);
        return format!("{}{}{}\x1b[0m", sz[2], sz[0], sz[1]);

    } else {
        return format!("{}{}{}\x1b[0m", sz[2], sz[0], sz[1]);
    }
}

pub fn display(map: HashMap<u64, String>) {
    let mut sizes: Vec<&u64> = map.keys().collect();
    sizes.sort();

    let mut sz_strs: Vec<[String; 3]> = vec![
        [String::new(), String::new(), String::new()]; 
        sizes.len()
    ];

    let mut file_names = vec![
        String::new(); sizes.len()
    ];

    let mut mx_len: usize = 0;
    for (idx, sz) in sizes.into_iter().enumerate() {
        let (trunc_sz, szstr, color_str) = getsz(sz);
        let apparent_len = trunc_sz.len();
        if apparent_len > mx_len {
            mx_len = apparent_len;
        }

        sz_strs[idx][0] = trunc_sz;
        sz_strs[idx][1] = szstr;
        sz_strs[idx][2] = color_str;

        let fname = map.get(sz).expect("issue getting val from hashmap");
        file_names[idx] = fname.to_owned();
    }
    mx_len += 3;

    for (i, sz) in sz_strs.iter().enumerate() {
        let fsz = fmtsz(sz);
        println!(
            " {}{}{}",
            fsz,
            " ".repeat(mx_len - sz[0].len()),
            file_names[i],
        );
    }
}