#![warn(clippy::all, clippy::pedantic)]
use plage::Plage;
mod plage;

fn cache(cd: bool) {
    let home: String;
    match std::env::var("HOME") {
        Ok(val) => home = val,
        _ => panic!(),
    }
    let mut cache = home;
    cache.push_str("/.cache/plage");

    if cd {
        if !std::path::Path::new(&cache).exists() {
            std::fs::create_dir(&cache)
                .expect("Failed to create cache directory");
        }
        std::env::set_current_dir(cache)
            .expect("Failed to open cache directory");
    } else {
        if !std::fs::remove_dir_all(cache).is_err() {
            println!("Cache does not exist");
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let args_len = args.len();
    let mut p: Plage = Plage {
        clone: false,
        build: false,
        install: false,
        remove: false,
        clean: false
    };

    if args_len == 1 {
        missing_args();
        return;
    }

    match args[1].as_str() {
        "--help" => {help(); return}
        "--version" => {version(); return}
        _ => (),
    }
    
    cache(true);

    match p.new(&args) {
        None => invalid_args(args[1].to_string()),
        Some(_) => println!("Plage filled"),
    }

    if p.remove {
        p.plage_remove(&args, args_len);
        return;
    }

    if p.clean {
        cache(false);
        return;
    }

    for i in 2..args_len {
        match p.plage_clone(&args, i) {
            Some(false) => return,
            Some(true) => println!("plage: clone successful"),
            None => (),
        }
        cache(true);
        match p.plage_build(&args, i) {
            Some(false) => return,
            Some(true) => println!("plage: build successful"),
            None => (),
        }
        cache(true);
        match p.plage_install(&args, i) {
            Some(false) => return,
            Some(true) => println!("plage: install successful"),
            None => (),
        }
        cache(true);
    }
}

fn missing_args() {
    println!("plage: not enough arguments");
    println!("Try 'plage --help'");
}

fn invalid_args(a: String) {
    println!("plage: invalid argument {}", a);
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
    println!("  l              list packages NAME");
    println!("  c              clean cache");
}
