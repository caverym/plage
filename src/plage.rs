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
    pub(crate) clone: bool,
    pub(crate) build: bool,
    pub(crate) install: bool,
    pub(crate) remove: bool,
    pub(crate) clean: bool,
}

impl Plage {
    pub fn new(&mut self, args: &[String]) -> Option<&Plage> {
        let ops: Vec<char> = args[1].chars().collect();
        let len: usize = ops.len();

        for i in 0..len {
            match ops[i] {
                'r' => self.remove = true,
                'c' => self.clean = true,
                'd' => self.clone = true,
                'b' => self.build = true,
                'i' => self.install = true,
                _ => return None,
            }
        }
        Some(self)
    }

    pub fn plage_remove(&self, args: &[String], len: usize) -> Option<bool> {
        let mut pacs: String = args[2].to_string();
        for i in 2..len {
            pacs.push_str(args[i].as_str());
            pacs.push(' ');
        }
        let ecode: std::process::ExitStatus =
        run("/usr/bin/pacman", "-Rs", pacs.as_str());
        if !ecode.success() {
            println!("pacman exit error");
            return Some(false);
        }
        Some(true)
    }

    pub fn plage_clone(&self, args: &[String], i: usize) -> Option<bool> {
        if !self.clone {return None}
        let mut act: Action = Action::Clone;
        if std::path::Path::new(&args[i]).exists() {
            act = Action::Update;
        }
        match act {
            Action::Clone => self.act_clone(args[i].as_str()),
            Action::Update => self.act_update(args[i].as_str()),
        }
    }

    fn act_clone(&self, package: &str) -> Option<bool> {
        let mut url: String = String::from("https://aur.archlinux.org/");
        url.push_str(package);
        url.push_str(".git");
        let ecode: std::process::ExitStatus = 
        run("/usr/bin/git", "clone", &url);
        
        if !ecode.success() {
            println!("git exit error");
            return Some(false);
        }
        Some(true)
    }

    fn act_update(&self, package: &str) -> Option<bool> {
        std::env::set_current_dir(package)
            .expect("Failed to change directory");
        let mut url: String = String::from("https://aur.archlinux.org/");
        url.push_str(package);
        url.push_str(".git");
        let ecode: std::process::ExitStatus =
        run("/usr/bin/git", "pull", &url);

        if !ecode.success() {
            println!("git exit error");
            return Some(false);
        }
        Some(true)
    }

    pub fn plage_build(&self, args: &[String], i: usize) -> Option<bool> {
        if !self.build {return None}
        if !std::path::Path::new(&args[i]).exists() {
            println!("{} does not exist", args[i]);
            return Some(false);
        }
        std::env::set_current_dir(args[i].as_str())
            .expect("failed to change directory");
        let ecode: std::process::ExitStatus =
        run("/usr/bin/makepkg", "-sf", args[i].as_str());
        if !ecode.success() {
            println!("makepkg exit error");
            return Some(false);
        }
        Some(true)
    }

    pub fn plage_install(&self, args: &[String], i: usize) -> Option<bool> {
        if !self.install {return None}
        if !std::path::Path::new(&args[i]).exists() {
            println!("{} does not exist", args[i]);
            return Some(false);
        }
        std::env::set_current_dir(args[i].as_str())
            .expect("failed to change directory");
        let ecode: std::process::ExitStatus =
        run("/usr/bin/makepkg", "-i", args[i].as_str());
        if !ecode.success() {
            println!("makepkg exit error");
            return Some(false);
        }
        Some(true)
    }
}
