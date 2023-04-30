pub struct DuxOptions {
    pub recurse: bool,
    pub color: bool,
    pub all: bool,
}

pub fn get_options() -> (DuxOptions, Vec<String>) {
    let mut opts = DuxOptions {
        recurse: false,
        color: true,
        all: false,
    };
    let mut paths = vec![];

    let args: Vec<String> = std::env::args().skip(1).collect();

    for arg in args {
        if arg.starts_with("-") && arg.len() > 1 {
            for c in arg.chars().skip(1) {
                match c {
                    'r' => opts.recurse = true,
                    'n' => opts.color = false,
                    'a' => opts.all = true,
                    _ => {}
                }
            }
            continue;
        }

        paths.push(arg);
    }

    if atty::isnt(atty::Stream::Stdout) {
        opts.color = false;
    }

    return (opts, paths);
}
