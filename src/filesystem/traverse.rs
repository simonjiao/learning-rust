use error_chain::error_chain;
use std::{env, fs};
use walkdir::WalkDir;

error_chain! {
    foreign_links {
        WalkDir(walkdir::Error);
        Io(std::io::Error);
        Time(std::time::SystemTimeError);
        Glob(glob::GlobError);
        Pattern(glob::PatternError);
    }
}

pub fn find_recent_modified_file(seconds: u64) -> Result<()> {
    let current_dir = env::current_dir()?;

    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();

        let metadata = fs::metadata(&path)?;
        let last_modified = metadata.modified()?.elapsed()?.as_secs();

        if last_modified < seconds && metadata.is_file() {
            println!(
                "Last modified {} seconds, is readonly {}, size {} bytes, filename {:?}",
                last_modified,
                metadata.permissions().readonly(),
                metadata.len(),
                path.file_name().ok_or("No filename").unwrap()
            );
        }
    }

    Ok(())
}

use same_file::is_same_file;
use std::path::{Path, PathBuf};

pub fn contains_loop<P: AsRef<Path>>(path: P) -> std::io::Result<Option<(PathBuf, PathBuf)>> {
    let path = path.as_ref();
    let mut path_buf = path.to_path_buf();
    while path_buf.pop() {
        if is_same_file(&path_buf, path)? {
            return Ok(Some((path_buf, path.to_path_buf())));
        } else if let Some(loop_paths) = contains_loop(&path_buf)? {
            return Ok(Some(loop_paths));
        }
    }
    Ok(None)
}

use std::collections::HashMap;

pub fn find_dup_filename() {
    let mut filenames = HashMap::new();

    for entry in WalkDir::new(".")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| !e.file_type().is_dir())
    {
        let f_name = String::from(entry.file_name().to_string_lossy());
        let counter = filenames.entry(f_name.clone()).or_insert(0);
        *counter += 1;

        if *counter == 2 {
            println!("{}", f_name);
        }
    }
}

pub fn find_by_ext(ext: &'static str) -> Result<()> {
    for entry in WalkDir::new(".")
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let f_name = entry.file_name().to_string_lossy();
        let sec = entry.metadata()?.modified()?;

        if f_name.ends_with(ext) && sec.elapsed()?.as_secs() < 86400 {
            println!("{}", f_name);
        }
    }

    Ok(())
}

use walkdir::DirEntry;

fn is_not_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| entry.depth() == 0 || !s.starts_with("."))
        .unwrap_or(false)
}

pub fn skip_dotfiles() {
    WalkDir::new(".")
        .into_iter()
        .filter_entry(|e| is_not_hidden(e))
        .filter_map(|v| v.ok())
        .for_each(|x| println!("{}", x.path().display()));
}

pub fn cal_size(min: usize, max: usize) {
    let total_size = WalkDir::new(".")
        .min_depth(min)
        .max_depth(max)
        .into_iter()
        .filter_map(|v| v.ok())
        .filter_map(|v| v.metadata().ok())
        .filter(|m| m.is_file())
        .fold(0, |acc, m| acc + m.len());

    println!("Total size {} bytes", total_size);

    //let v = vec![1, 2, 3, 4, 5];
    //println!("{}", v.iter().fold(1, |acc, m| acc * m));
}

use glob::glob;

pub fn find_json() -> Result<()> {
    for entry in glob("**/*.json")? {
        println!("{}", entry?.display());
    }

    Ok(())
}

use glob::{glob_with, MatchOptions};

pub fn glob_with_options() -> Result<()> {
    let options = MatchOptions {
        case_sensitive: false,
        ..Default::default()
    };

    for entry in glob_with("/media/*.png", options)? {
        println!("{}", entry?.display());
    }

    Ok(())
}

#[allow(dead_code)]
fn default_fn() {
    #[derive(Debug)]
    struct Example {
        a: u32,
        b: u32,
        c: u32,
    }
    impl Example {
        fn default() -> Self {
            Self { a: 1, b: 4, c: 5 }
        }
    }

    let e = Example {
        a: 3,
        c: 6,
        ..Example::default()
    };
    println!("{:?}", e);
}
