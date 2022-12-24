use std::{
    borrow::Cow,
    fs::{read_dir, DirEntry, FileType, ReadDir},
    io::{self, Write},
    path::Path,
};

use clap::{Arg, ArgAction, Command};

const BLUE: &str = "\u{001b}[34m";
const RESET: &str = "\u{001b}[0m";
const DIR_WORD: &str = "directories";
const FILE_WORD: &str = "files";

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
        path: &Path,
        depth: &mut usize,
        include_hidden: bool,
    ) -> io::Result<()> {
        let contents = read_dir(path)?;
        *depth += 1;
        self.read_target(contents, depth, path, include_hidden)
    }

    fn read_target(
        &mut self,
        t: ReadDir,
        depth: &mut usize,
        path: &Path,
        include_hidden: bool,
    ) -> io::Result<()> {
        for match_dir in t {
            match match_dir {
                Ok(dir_entry) => {
                    self.get_content_type(dir_entry, depth, path, include_hidden)?;
                }
                Err(why) => eprintln!("{}", why),
            }
        }
        Ok(())
    }

    fn get_content_type(
        &mut self,
        dir: DirEntry,
        depth: &mut usize,
        path: &Path,
        include_hidden: bool,
    ) -> io::Result<()> {
        match dir.file_type() {
            Ok(ftype) => self.read_crawled_content(
                ftype,
                depth,
                path,
                dir.file_name().to_string_lossy(),
                include_hidden,
            ),
            Err(why) => panic!("{}", why),
        }
    }

    fn read_crawled_content(
        &mut self,
        ftype: FileType,
        depth: &mut usize,
        path: &Path,
        target_name: Cow<str>,
        include_hidden: bool,
    ) -> io::Result<()> {
        if ftype.is_dir() {
            let inner_path = path.join(target_name.to_string());
            if target_name.starts_with('.') && !include_hidden {
                return Ok(());
            }
            self.display_dir(&target_name, &inner_path, depth, include_hidden)?;
            return Ok(());
        }

        if target_name.starts_with('.') && !include_hidden {
            Ok(())
        } else {
            self.display_file(&target_name, depth)?;
            Ok(())
        }
    }

    fn display_dir(
        &mut self,
        target_name: &str,
        inner_path: &Path,
        depth: &mut usize,
        include_hidden: bool,
    ) -> io::Result<()> {
        self.total_dirs += 1;
        let mut inner_depth = *depth;

        writeln!(
            io::stdout(),
            "{}{}└── {}{}",
            &" ".repeat(inner_depth),
            BLUE,
            target_name,
            RESET
        )?;
        io::stdout().flush()?;
        self.crawl_target(inner_path, &mut inner_depth, include_hidden)
    }

    fn display_file(&mut self, target_name: &str, depth: &mut usize) -> io::Result<()> {
        self.total_files += 1;
        writeln!(io::stdout(), "{}└── {}", &" ".repeat(*depth), target_name)?;
        io::stdout().flush()
    }

    fn display_result(&self) -> Result<(), io::Error> {
        let dir_word = if self.total_dirs > 1 {
            DIR_WORD
        } else {
            "directory"
        };
        let file_word = if self.total_files > 1 {
            FILE_WORD
        } else {
            "file"
        };
        writeln!(
            io::stdout(),
            "Total: {} {} and {} {}",
            self.total_dirs,
            dir_word,
            self.total_files,
            file_word,
        )
    }
}

fn main() -> io::Result<()> {
    let matches = Command::new("lls")
        .version("1.0")
        .author("Benjie Ben Garcia <mystique09>")
        .about("Displays directories as trees (with optional color/HTML output)")
        .arg(
            Arg::new("directory")
                .help("The directory tree to display")
                .index(1)
                .required(false)
                .default_value("."),
        )
        .arg(
            Arg::new("hidden")
                .short('a')
                .long("all")
                .help("Shows hidden files (those starting with a dot) as well")
                .required(false)
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let mut tree = Tree::new();
    let path = Path::new::<String>(matches.get_one("directory").unwrap());
    let include_hidden: &bool = matches.get_one::<bool>("hidden").unwrap_or(&false);
    let mut depth = 0;

    tree.crawl_target(path, &mut depth, *include_hidden)?;
    tree.display_result()?;

    Ok(())
}
