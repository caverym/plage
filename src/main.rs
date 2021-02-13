use std::{env, process::ExitStatus};
use std::process::Command;

struct Plage {
    clone: bool,
    build: bool,
    install: bool,
    remove: bool,
    list: bool,
    clean: bool,
}

fn find_ops(args: &Vec<String>) -> Plage {
    let ops: Vec<char> = args[1].chars().collect();
    let len = ops.len();
    let mut clone: bool = false;
    let mut build: bool = false;
    let mut install: bool = false;
    let mut remove: bool = false;
    let mut list: bool = false;
    let mut clean: bool = false;

    for i in 0..len {
        match ops[i] {
            'l' => {
                list = true; 
                break;
            }
            'r' => {
                remove = true;
                break;
            }
            'c' => clean = true,
            'd' => clone = true,
            'b' => build = true,
            'i' => install = true,
            _ => invalid_args(args[1].to_string()),
        }
    }

    let p = Plage {
        clone: clone,
        build: build,
        install: install,
        remove: remove,
        list: list,
        clean: clean,
    };

    return p;
}

fn cache(cd: bool) {
    let home;
    match env::var("HOME") {
        Ok(val) => home = val,
        _ => panic!(),
    }
    let mut cache = home;
    cache.push_str("/.cache/plage");

    match cd {
        true => {
            if std::path::Path::new(&cache).exists() == false {
                std::fs::create_dir(&cache).expect("Failed to create cache directory");
            }
            std::env::set_current_dir(cache).expect("Failed to open cache directory");
        }

        false => {
            if !std::fs::remove_dir_all(cache).is_err() {
                println!("Cache does not exist");
            }
        }
    }
}

fn run(path: &str, ar1: &str, ar2: &str) -> ExitStatus {
    let mut child = Command::new(path)
        .arg(ar1)
        .arg(ar2)
        .spawn()
        .expect("failed to execute Git");
    let ecode = child.wait().expect("Failed in wait");
    return ecode;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let args_len = args.len();
    let plage: Plage;

    match args[1].as_str() {
        "--help" => {help(); return}
        "--version" => {version(); return}
        _ => plage = find_ops(&args),
    }

    if plage.clean {
        cache(false);
    }

    if args.len() <= 2{
        missing_args();
        return;
    }

    cache(true);

    if plage.list {
        run("/usr/bin/pacman", "-Qs", args[2].as_str());
        return;
    }

    if plage.remove {
        let mut pacs: String = args[2].to_string();
        for i in 3..args_len {
            pacs.push(' ');
            pacs.push_str(args[i].as_str());
        }
        run("/usr/bin/pacman", "-Rsu", pacs.as_str());
        return;
    }

    for i in 2..args_len {
        if plage.clone {
            if std::path::Path::new(&args[i]).exists() {
                println!("{} already cloned", args[i]);
            }
            let mut url = String::from("https://aur.archlinux.org/");
            url.push_str(args[i].as_str());
            url.push_str(".git");
            let ecode = run("/usr/bin/git", "clone", url.as_str());
            if ecode.success() == false {
                println!("plage: git exited with an error");
            }
            cache(true);
        }

        if plage.build {
            if std::path::Path::new(&args[i]).exists() == false {
                println!("{} does not exist", args[i]);
            }
            std::env::set_current_dir(args[i].as_str())
                .expect("Failed to change directory");
            let ecode = run("/usr/bin/makepkg", "-sf", args[i].as_str());
            if ecode.success() == false {
                println!("makepkg failed");
                return;
            }
            cache(true);
        }

        if plage.install {
            if std::path::Path::new(&args[i]).exists() == false {
                println!("{} does not exist", args[i]);
            }
            std::env::set_current_dir(args[i].as_str())
                .expect("Failed to change directory");
            let ecode = run("/usr/bin/makepkg", "-i", args[i].as_str());
            if ecode.success() == false {
                println!("makepkg failed");
                return;
            }
            cache(true);
        }
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
    println!("Usage: plage [-d, -b, -i] [NAME...]\n");
    println!("  -d              downloads packages NAME");
    println!("  -b              builds packages NAME");
    println!("  -i              installs packages NAME");
    println!("  -r              removes packages NAME");
    println!("  -l              list packages NAME");
    println!("  -c              clean cache");
}