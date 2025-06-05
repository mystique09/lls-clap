use std::{
    borrow::Cow,
    fs::{read_dir, DirEntry, FileType, ReadDir},
};

use clap::{Arg, ArgAction, Command};

static BLUE: &str = "\u{001b}[34m";
static RESET: &str = "\u{001b}[0m";

fn main() {
    let app = Command::new("lls")
        .about("A tree command clone made with rust, displays the contents of a directory in a tree format.")
        .author("Benjie Ben, mystique09")
        .version("0.1.0")
        .args([
            Arg::new("all")
                .help("Include hidden files")
                .short('a')
                .long("all")
                .required(false)
                .action(ArgAction::SetTrue),
            Arg::new("target")
                .help("Target directory")
                .required(false)
                .action(ArgAction::Set)
                .num_args(1..),
        ])
        .get_matches();

    let mut depth = 0;

    let include_hidden: &bool = app.get_one::<bool>("all").unwrap_or(&false);
    let default_target = String::from(".");
    let target: &str = app.get_one::<String>("target").unwrap_or(&default_target);

    println!("{}", &target);
    let mut tree = Tree::new();
    match tree.crawl_target(target, &mut depth, include_hidden) {
        Ok(_) => tree.display_result(),
        Err(why) => println!("{why}"),
    };
}

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

    fn crawl_target(
        &mut self,
        path: &str,
        depth: &mut usize,
        include_hidden: &bool,
    ) -> Result<(), std::io::Error> {
        let contents = read_dir(path)?;
        *depth += 1;

        self.read_target(contents, depth, path, include_hidden);
        Ok(())
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
            Ok(ftype) => self.display(
                ftype,
                depth,
                path,
                dir.file_name().to_string_lossy(),
                include_hidden,
            ),
            Err(why) => eprintln!("{why}"),
        }
    }

    fn display(
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
            self.display_dir(&target_name, &inner_path, depth, include_hidden);
        } else if ftype.is_file() {
            if target_name.starts_with('.') && !include_hidden {
                return;
            }
            self.display_file(&target_name, depth);
        }
    }

    fn display_dir(
        &mut self,
        target_name: &str,
        inner_path: &str,
        depth: &mut usize,
        include_hidden: &bool,
    ) {
        self.total_dirs += 1;
        let mut inner_depth = *depth;

        if *depth == 1 {
            println!("{}├── {}{}", &BLUE, target_name, &RESET);
        } else {
            println!(
                "{}{}├── {}{}",
                &" ".repeat(inner_depth * 2),
                &BLUE,
                target_name,
                &RESET
            );
        }

        match self.crawl_target(inner_path, &mut inner_depth, include_hidden) {
            Ok(_) => (),
            Err(why) => println!("{why}"),
        };
    }

    fn display_file(&mut self, target_name: &str, depth: &mut usize) {
        self.total_files += 1;
        if *depth == 1 {
            println!("├── {}", target_name);
        } else {
            println!("{}├── {}", &" ".repeat(*depth * 2), target_name);
        }
    }

    fn display_result(&self) {
        let dir_count = if self.total_dirs > 1 {
            format!("{} directories", &self.total_dirs)
        } else {
            format!("{} directory", &self.total_dirs)
        };

        let file_count = if self.total_files > 1 {
            format!("{} files", &self.total_files)
        } else {
            format!("{} file", &self.total_files)
        };

        println!("{}, {}", dir_count, file_count);
    }
}