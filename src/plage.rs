// plage struct

enum Action {
    Clone,
    Update,
}

fn run(path: &str, ar1: &str, ar2: &str) -> std::process::ExitStatus {
    let mut child: std::process::Child = std::process::Command::new(path)
        .arg(ar1)
        .arg(ar2)
        .spawn()
        .expect("failed to execute");
    child.wait().expect("failed to wait")
}

pub struct Plage {
    pub(crate) args: Vec<String>,
    pub(crate) length: usize,
    pub(crate) clone: bool,
    pub(crate) build: bool,
    pub(crate) install: bool,
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
                _ => return Err(()),
            }
        }
        Ok(self)
    }

    pub fn plage_clone(&self, i: usize) -> Option<bool> {
        if !self.clone {return None}
        let mut act: Action = Action::Clone;
        if std::path::Path::new(&self.args[i]).exists() {
            act = Action::Update;
        }
        match act {
            Action::Clone => Some(self.act_clone(self.args[i].as_str())),
            Action::Update => Some(self.act_update(self.args[i].as_str())),
        }
    }

    fn act_clone(&self, package: &str) -> bool {
        let mut url: String = String::from("https://aur.archlinux.org/");
        url.push_str(package);
        url.push_str(".git");
        let ecode: std::process::ExitStatus = 
        run("/usr/bin/git", "clone", &url);
        debug_assert!(ecode.success());
        if !ecode.success() {
            println!("git exit error");
            return false;
        }
        true
    }

    fn act_update(&self, package: &str) -> bool {
        std::env::set_current_dir(package)
            .expect("Failed to change directory");
        let mut url: String = String::from("https://aur.archlinux.org/");
        url.push_str(package);
        url.push_str(".git");
        let ecode: std::process::ExitStatus =
        run("/usr/bin/git", "pull", &url);
        debug_assert!(ecode.success());
        if !ecode.success() {
            println!("git exit error");
            return false;
        }
        true
    }

    pub fn plage_build(&self, i: usize) -> Option<bool> {
        if !self.build {return None}
        if !std::path::Path::new(&self.args[i]).exists() {
            println!("{} does not exist", self.args[i]);
            return Some(false);
        }
        std::env::set_current_dir(self.args[i].as_str())
            .expect("failed to change directory");
        let ecode: std::process::ExitStatus =
        run("/usr/bin/makepkg", "-sf", self.args[i].as_str());
        debug_assert!(ecode.success());
        if !ecode.success() {
            println!("makepkg exit error");
            return Some(false);
        }
        Some(true)
    }

    pub fn plage_install(&self, i: usize) -> Option<bool> {
        if !self.install {return None}
        if !std::path::Path::new(&self.args[i]).exists() {
            println!("{} does not exist", self.args[i]);
            return Some(false);
        }
        std::env::set_current_dir(self.args[i].as_str())
            .expect("failed to change directory");
        let ecode: std::process::ExitStatus =
        run("/usr/bin/makepkg", "-i", self.args[i].as_str());
        debug_assert!(ecode.success());
        if !ecode.success() {
            println!("makepkg exit error");
            return Some(false);
        }
        Some(true)
    }
}
