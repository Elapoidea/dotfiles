use std::process::Command;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const DATA_FILE: &str = "/home/dea/.config/rustwal/src/.paths";
const SUBL_WAL: &str = "/home/dea/pywal_sublime/pywal_sublime.py";

pub struct Themes {
    pub paths: Vec<String>
}

impl Themes {
    pub fn new() -> Themes {
        Themes { paths: read_paths() }
    }

    pub fn use_theme(self, index: usize) {
        pywal(&self.paths[index]);
     
    }

    pub fn apply_to_apps(self) {
        firefox();
        subl();
    }

    pub fn add_path(mut self, path: String) {
        self.paths.push(path);

        write_file(&self.paths.join("\n"), DATA_FILE);
    }

    pub fn del_path(mut self, index: usize) {
        self.paths.remove(index);

        write_file(&self.paths.join("\n"), DATA_FILE);
    }

    pub fn search_paths(self, folder: String) {
        for (i, path) in self.paths.iter().enumerate() {
            if path.starts_with(&folder) { println!("{}: {}", i + 1, path) }
        }
    }
}

fn pywal(wall: &String) {
    Command::new("wal")
        .arg("--vte")
        .arg("-qti")
        .arg(wall)
        .spawn()
        .unwrap();

}

fn firefox() {
    Command::new("pywalfox")
        .arg("update")
        .spawn()
        .unwrap();
}

fn subl() {
    Command::new("python3")
        .arg(SUBL_WAL)
        .spawn()
        .unwrap();
}

fn read_paths() -> Vec<String> {
    let mut palletes: Vec<String> = vec![];

    for path in read_file(DATA_FILE).lines() {
        palletes.push(path.to_string())
    }

    palletes
}

fn read_file(path: &str) -> String {
    let mut file = File::open(Path::new(&path)).unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();

    s
}

fn write_file(data: &str, path: &str) {
    let mut file = File::create(Path::new(path)).unwrap();

    file.write_all(&data.as_bytes()).unwrap();
}
