#![warn(clippy::all, clippy::pedantic)]
mod plage;

fn cache() {
    let home: String;
    match std::env::var("HOME") {
        Ok(val) => home = val,
        Err(e) => panic!("{}", e),
    }

    let cache = format!("{}/.cache/plage", home);

    if !std::path::Path::new(&cache).exists() {
        std::fs::create_dir(&cache).expect("Failed to create cache directory");
    }
    std::env::set_current_dir(cache).expect("Failed to open cache directory");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let len = args.len();
    let mut p: plage::Plage = plage::Plage {
        args,
        length: len,
        clone: false,
        build: false,
        install: false,
        verbose: false,
    };

    if p.length == 1 {
        missing_args();
        return;
    }

    match p.args[1].as_str() {
        "--help" | "-h" => {
            help();
            return;
        }
        "--version" | "-v" => {
            version();
            return;
        }
        _ => (),
    }

    if p.new().is_err() {
        invalid_args(&p.args[1]);
        return;
    }
    if p.verbose {
        println!("Plage initiated\n{:?}", p)
    }

    if users::get_current_gid() == 0 {
        println!("cannot run as root");
        return;
    }

    if p.verbose {
        println!("Change to cache directory")
    }
    cache();

    if p.verbose {
        println!("Running main loop")
    }
    for i in 2..p.length {
        if p.verbose {
            println!("loop #{}", i - 1)
        }
        match p.plage_clone(i) {
            Some(false) => return,
            Some(true) => cache(),
            None => (),
        }
        match p.plage_build(i) {
            Some(false) => return,
            Some(true) => cache(),
            None => (),
        }
        match p.plage_install(i) {
            Some(false) => return,
            Some(true) => cache(),
            None => (),
        }
    }
    if p.verbose {
        println!("exited main loop")
    }
}

fn missing_args() {
    println!("plage: not enough arguments");
    println!("Try 'plage --help'");
}

fn invalid_args(a: &str) {
    println!("plage: invalid argument '{}'", a);
    println!("Try 'plage --help'");
}

fn version() {
    println!("Plage 1.0");
}

fn help() {
    println!("Usage: plage [d, db, dbi, ...] [NAME...]\n");
    println!("  a              download, build & install packages NAME");
    println!("  A              same as `a` but verbose");
    println!("  b              builds packages NAME");
    println!("  d              downloads packages NAME");
    println!("  i              installs packages NAME");
    println!("  v              use verbose output");
}
