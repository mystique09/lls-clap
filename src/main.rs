use std::{
    borrow::Cow,
    fs::{read_dir, DirEntry, FileType, ReadDir},
    io::Write,
};

use clap::{Arg, ArgAction, Command};

static BLUE: &str = "\u{001b}[34m";
static RESET: &str = "\u{001b}[0m";

struct Tree {
    total_files: usize,
    total_dirs: usize,
}

impl Tree {
    fn new() -> Self {
        Self {
            total_files: 0,
            total_dirs: 0,
        }
    }

    fn crawl_target(&mut self, path: &str, depth: &mut usize, include_hidden: &bool) {
        let contents = read_dir(&path);
        *depth += 1;

        match contents {
            Ok(content) => self.read_target(content, depth, path, include_hidden),
            Err(why) => eprintln!("{why}"),
        }
    }

    fn read_target(&mut self, t: ReadDir, depth: &mut usize, path: &str, include_hidden: &bool) {
        for match_dir in t {
            match match_dir {
                Ok(dir_entry) => self.get_content_type(dir_entry, depth, path, include_hidden),
                Err(why) => eprintln!("{why}"),
            }
        }
    }

    fn get_content_type(
        &mut self,
        dir: DirEntry,
        depth: &mut usize,
        path: &str,
        include_hidden: &bool,
    ) {
        match dir.file_type() {
            Ok(ftype) => self.read_crawled_content(
                ftype,
                depth,
                path,
                dir.file_name().to_string_lossy(),
                include_hidden,
            ),
            Err(why) => eprintln!("{why}"),
        }
    }

    fn read_crawled_content(
        &mut self,
        ftype: FileType,
        depth: &mut usize,
        path: &str,
        target_name: Cow<str>,
        include_hidden: &bool,
    ) {
        if ftype.is_dir() {
            if target_name.starts_with('.') && !include_hidden {
                return;
            }
            let inner_path = format!("{}/{}", path, target_name);
            self.display_dir(&inner_path, depth, include_hidden);
        }

        if ftype.is_file() {
            if target_name.starts_with('.') && !include_hidden {
                return;
            }
            self.display_file(&target_name, depth);
        }
    }

    fn display_dir(&mut self, target_name: &str, depth: &mut usize, include_hidden: &bool) {
        self.total_dirs += 1;
        let mut inner_depth = *depth;

        println!(
            "{}{BLUE}└── {}{RESET}",
            &" ".repeat(inner_depth),
            target_name.replace("./", "")
        );
        std::io::stdout().flush().unwrap();
        self.crawl_target(target_name, &mut inner_depth, include_hidden);
    }

    fn display_file(&mut self, target_name: &str, depth: &mut usize) {
        self.total_files += 1;
        println!("{}└── {}", &" ".repeat(*depth), target_name);
        std::io::stdout().flush().unwrap();
    }

    fn display_result(&self) {
        let dir_word = if self.total_dirs > 1 {
            "directories"
        } else {
            "directory"
        };

        let file_word = if self.total_files > 1 {
            "files"
        } else {
            "file"
        };

        println!(
            "{} {dir_word}, {} {file_word}",
            self.total_dirs, self.total_files
        );
        std::io::stdout().flush().unwrap();
    }
}

fn main() {
    let app = Command::new("lls")
        .about("A tree command clone made with rust, displays the contents of a directory in a tree format.")
        .author("Benjie Ben, mystique09")
        .version("0.1.0")
        .args([
            Arg::new("all")
                .short('a')
                .long("all")
                .required(false)
                .action(ArgAction::SetTrue),
            Arg::new("target")
                .help("target directory")
                .required(false)
                .action(ArgAction::Set)
                .num_args(1..),
        ])
        .get_matches();

    let mut depth = 0;

    let include_hidden: &bool = app.get_one::<bool>("all").unwrap_or(&false);
    let default_target = String::from(".");
    let target: &str = app.get_one::<String>("target").unwrap_or(&default_target);

    println!("{}", target);
    let mut tree = Tree::new();
    tree.crawl_target(target, &mut depth, include_hidden);
    tree.display_result();
}
