#![warn(clippy::all, clippy::pedantic)]
mod plage;
use lliw::Fg;
use std::{env, fs, path};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut plage = plage::Plage::new(args);

    if plage.length == 1 {
        missing_args();
        return;
    }

    match plage.args[1].as_str() {
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

    if plage.get_actions().is_err() {
        invalid_args(&plage.args[1]);
        return;
    }

    verbose_println!(
        plage.verbose,
        "{}Plage:{} initiated Plage",
        Fg::Cyan,
        Fg::Reset
    );

    if users::get_current_gid() == 0 {
        eprintln!("{}Error:{} cannot run as root", Fg::Red, Fg::Reset);
        return;
    }

    cache();
    verbose_println!(
        plage.verbose,
        "{}Plage:{} changed to cache directory",
        Fg::Cyan,
        Fg::Reset
    );
    verbose_println!(
        plage.verbose,
        "{}Plage:{} Starting main loop",
        Fg::Cyan,
        Fg::Reset
    );
    for i in 2..plage.length {
        verbose_println!(plage.verbose, "{}Loop: #{}{}", Fg::Red, i - 1, Fg::Reset);
        match plage.plage_clone(i) {
            Some(false) => return,
            Some(true) => cache(),
            None => (),
        }
        match plage.plage_build(i) {
            Some(false) => return,
            Some(true) => cache(),
            None => (),
        }
        match plage.plage_install(i) {
            Some(false) => return,
            Some(true) => cache(),
            None => (),
        }
    }
    verbose_println!(
        plage.verbose,
        "{}Plage:{} exited main loop",
        Fg::Cyan,
        Fg::Reset
    );
}

fn cache() {
    let home: String;
    match env::var("HOME") {
        Ok(val) => home = val,
        Err(e) => panic!("{}", e),
    }

    let cache = format!("{}/.cache/plage", home);

    if !path::Path::new(&cache).exists() {
        fs::create_dir(&cache).expect("Failed to create cache directory");
    }
    std::env::set_current_dir(cache).expect("Failed to open cache directory");
}

fn missing_args() {
    println!("{}Plage:{} not enough arguments", Fg::Cyan, Fg::Reset);
    println!("Try 'plage --help'");
}

fn invalid_args(a: &str) {
    println!("{}Plage:{} invalid argument '{}'", Fg::Cyan, Fg::Reset, a);
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
