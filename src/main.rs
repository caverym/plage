#![warn(clippy::all, clippy::pedantic)]
use plage::Plage;
use users::get_current_gid;
mod plage;

fn cache() {
    let home: String;
    match std::env::var("HOME") {
        Ok(val) => home = val,
        Err(e) => panic!("{}", e),
    }

    let mut cache = home;
    cache.push_str("/.cache/plage");

    if !std::path::Path::new(&cache).exists() {
        std::fs::create_dir(&cache)
            .expect("Failed to create cache directory");
    }
    std::env::set_current_dir(cache)
        .expect("Failed to open cache directory");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let len = args.len();
    let mut p: Plage = Plage {
        args,
        length: len,
        clone: false,
        build: false,
        install: false
    };

    if p.length == 1 {
        missing_args();
        return;
    }

    match p.args[1].as_str() {
        "--help" => {help(); return}
        "--version" => {version(); return}
        _ => println!("filling Plage..."),
    }

    if p.new().is_err() {
        invalid_args(&p.args[1]);
        return;
    }

    if users::get_current_gid() == 0 {
        println!("cannot run as root");
        return;
    }

    cache();
    
    for i in 2..p.length {
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
    println!("  d              downloads packages NAME");
    println!("  b              builds packages NAME");
    println!("  i              installs packages NAME");
    println!("  r              removes packages NAME");
}
