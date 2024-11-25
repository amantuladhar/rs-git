use std::fs;

pub fn init() {
    fs::create_dir(".git").expect("unable to create .git directory");
    fs::create_dir(".git/objects").expect("unable to create .git/objects");
    fs::create_dir(".git/refs").expect("unable to create .git/refs");
    fs::write(".git/HEAD", "ref: refs/heads/main\n").expect("unable to write to HEAD files");
    println!("Initialized git directory")
}
