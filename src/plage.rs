#[derive(Debug)]
pub struct Plage {
    pub(crate) args: Vec<String>,
    pub(crate) length: usize,
    pub(crate) clone: bool,
    pub(crate) build: bool,
    pub(crate) install: bool,
    pub(crate) verbose: bool,
}

fn run(path: &str, ar1: &str, ar2: &str) -> std::process::ExitStatus {
    let mut child: std::process::Child = std::process::Command::new(path)
        .args(&[ar1, ar2])
        .spawn()
        .expect("failed to execute");
    child.wait().expect("failed to wait")
}

impl Plage {
    pub fn new(&mut self) -> Result<&Plage, ()> {
        let ops: Vec<char> = self.args[1].chars().collect();
        let len: usize = ops.len();

        for i in 0..len {
            match ops.get(i).unwrap() {
                'd' => self.clone = true,
                'b' => self.build = true,
                'i' => self.install = true,
                'v' => self.verbose = true,

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
                _ => return Err(()),
            }
        }
        if self.verbose {
            println!("verbose mode enabled")
        }
        Ok(self)
    }

    pub fn plage_clone(&self, i: usize) -> Option<bool> {
        if !self.clone {
            if self.verbose {
                println!("`plage_clone` returns None")
            }
            return None;
        }
        let mut act: bool = true;
        if std::path::Path::new(&self.args[i]).exists() {
            act = false;
        }

        if act {
            return Some(self.act_clone(&self.args[i]));
        }
        Some(self.act_update(&self.args[i]))
    }

    fn act_clone(&self, package: &str) -> bool {
        let mut url: String = String::from("https://aur.archlinux.org/");
        url.push_str(package);
        url.push_str(".git");
        if self.verbose {
            println!("launching git")
        }
        let ecode: std::process::ExitStatus = run("/usr/bin/git", "clone", &url);
        debug_assert!(ecode.success());
        if !ecode.success() {
            println!("git exit error");
            return false;
        }
        true
    }

    fn act_update(&self, package: &str) -> bool {
        std::env::set_current_dir(package).expect("Failed to change directory");
        if self.verbose {
            println!("launching git")
        }
        let ecode: std::process::ExitStatus = run("/usr/bin/git", "pull", "--rebase");
        debug_assert!(ecode.success());
        if !ecode.success() {
            println!("git exit error");
            return false;
        }
        true
    }

    pub fn plage_build(&self, i: usize) -> Option<bool> {
        if !self.build {
            if self.verbose {
                println!("`plage_build` returns none")
            }
            return None;
        }
        if !std::path::Path::new(&self.args[i]).exists() {
            println!("{} does not exist", self.args[i]);
            return Some(false);
        }
        std::env::set_current_dir(self.args[i].as_str()).expect("failed to change directory");
        if self.verbose {
            println!("launching makepkg")
        }
        let ecode: std::process::ExitStatus = run("/usr/bin/makepkg", "-sf", self.args[i].as_str());
        debug_assert!(ecode.success());
        if !ecode.success() {
            println!("makepkg exit error");
            return Some(false);
        }
        Some(true)
    }

    pub fn plage_install(&self, i: usize) -> Option<bool> {
        if !self.install {
            if self.verbose {
                println!("`plage_install` returns none")
            }
            return None;
        }
        if !std::path::Path::new(&self.args[i]).exists() {
            println!("{} does not exist", self.args[i]);
            return Some(false);
        }
        std::env::set_current_dir(self.args[i].as_str()).expect("failed to change directory");
        if self.verbose {
            println!("launching makepkg")
        }
        let ecode: std::process::ExitStatus = run("/usr/bin/makepkg", "-i", self.args[i].as_str());
        debug_assert!(ecode.success());
        if !ecode.success() {
            println!("makepkg exit error");
            return Some(false);
        }
        Some(true)
    }
}
