extern crate git2;
use lliw::Fg;
use std::{env, path, process};

#[derive(Debug)]
pub struct Plage {
    pub args: Vec<String>,
    pub length: usize,
    pub clone: bool,
    pub build: bool,
    pub install: bool,
    pub verbose: bool,
    pub search: bool,
}

#[macro_export]
macro_rules! verbose_println {
    ($v:expr, $fmt:literal, $( $arg:expr ),*) => {
        if $v { println!($fmt, $( $arg ),*) }
    };
    ($v:expr, $fmt:literal) => {
        if $v { println!($fmt) }
    };
}

fn run(path: &str, ar1: &str, ar2: &str) -> process::ExitStatus {
    let mut child: process::Child = process::Command::new(path)
        .args(&[ar1, ar2])
        .spawn()
        .expect("failed to execute");
    child.wait().expect("failed to wait")
}

impl Plage {
    pub fn new(args: Vec<String>) -> Plage {
        let len: usize = args.len();
        Plage {
            args,
            length: len,
            clone: false,
            build: false,
            install: false,
            verbose: false,
            search: false,
        }
    }

    pub fn get_actions(&mut self) -> Result<&Plage, ()> {
        let ops: Vec<char> = self.args[1].chars().collect();
        let len: usize = ops.len();

        for i in 0..len {
            match ops.get(i).unwrap() {
                'a' => {
                    self.clone = true;
                    self.build = true;
                    self.install = true;
                    //verbose already false
                    break;
                }

                'A' => {
                    self.clone = true;
                    self.build = true;
                    self.install = true;
                    self.verbose = true;
                    break;
                }

                's' => {
                    self.search = true;
                    break;
                }

                'b' => self.build = true,
                'd' => self.clone = true,
                'i' => self.install = true,
                'v' => self.verbose = true,
                _ => return Err(()),
            }
        }
        verbose_println!(
            self.verbose,
            "{}Plage:{} verbose mode enabled",
            Fg::Cyan,
            Fg::Reset
        );
        Ok(self)
    }

    pub fn plage_clone(&self, i: usize) -> Option<bool> {
        if !self.clone {
            verbose_println!(
                self.verbose,
                "{}Plage:{} `plage_clone` returns None",
                Fg::Cyan,
                Fg::Reset
            );
            return None;
        }
        if !path::Path::new(&self.args[i]).exists() {
            return Some(self.act_clone(&self.args[i]));
        }
        Some(self.act_update(&self.args[i]))
    }

    fn act_clone(&self, package: &str) -> bool {
        let url: String = format!("https://aur.archlinux.org/{}.git", package);
        verbose_println!(
            self.verbose,
            "{}Plage:{} launching git",
            Fg::Cyan,
            Fg::Reset
        );
        if !run("/usr/bin/git", "clone", &url).success() {
            println!("{}Error:{} git exit error", Fg::Red, Fg::Reset);
            return false;
        }
        true
    }

    fn act_update(&self, package: &str) -> bool {
        if env::set_current_dir(package).is_err() {
            eprintln!(
                "{}Error:{} failed to open package directory",
                Fg::Red,
                Fg::Reset
            );
            return false;
        }
        verbose_println!(
            self.verbose,
            "{}Plage:{} launching git",
            Fg::Cyan,
            Fg::Reset
        );
        if !run("/usr/bin/git", "pull", "--rebase").success() {
            eprintln!("{}Error:{} git exit error", Fg::Red, Fg::Reset);
            return false;
        }
        true
    }

    pub fn plage_build(&self, i: usize) -> Option<bool> {
        if !self.build {
            verbose_println!(
                self.verbose,
                "{}Plage:{} `plage_build` returns none",
                Fg::Cyan,
                Fg::Reset
            );
            return None;
        }
        if !path::Path::new(&self.args[i]).exists() {
            eprintln!(
                "{}Plage:{} {} does not exist",
                Fg::Cyan,
                Fg::Reset,
                self.args[i]
            );
            return Some(false);
        }
        env::set_current_dir(self.args[i].as_str()).expect("failed to change directory");
        verbose_println!(
            self.verbose,
            "{}Plage:{} launching makepkg",
            Fg::Cyan,
            Fg::Reset
        );
        if !run("/usr/bin/makepkg", "-sf", self.args[i].as_str()).success() {
            println!("{}Error:{} makepkg exit error", Fg::Red, Fg::Reset);
            return Some(false);
        }
        Some(true)
    }

    pub fn plage_install(&self, i: usize) -> Option<bool> {
        if !self.install {
            verbose_println!(
                self.verbose,
                "{}Plage:{} `plage_install returns none",
                Fg::Cyan,
                Fg::Reset
            );
            return None;
        }
        if !path::Path::new(&self.args[i]).exists() {
            eprintln!(
                "{}Plage:{} {} does not exist",
                Fg::Cyan,
                Fg::Reset,
                self.args[i]
            );
            return Some(false);
        }
        std::env::set_current_dir(self.args[i].as_str()).expect("failed to change directory");
        verbose_println!(
            self.verbose,
            "{}Plage:{} launching makepkg",
            Fg::Cyan,
            Fg::Reset
        );
        if !run("/usr/bin/makepkg", "-i", self.args[i].as_str()).success() {
            eprintln!("{}Error:{} makepkg exit error", Fg::Red, Fg::Reset);
            return Some(false);
        }
        Some(true)
    }
}
