use std::env;
use std::fs::{self, DirEntry, Metadata};
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use std::cmp::Ordering;
use chrono;

#[derive(Default)]
struct Options {
    show_hidden: bool,
    long_format: bool,
    directories_only: bool,
    sort_by_time: bool,
    reverse_order: bool,
    recursive: bool,
    access_time: bool,
    show_type: bool,
    unsorted: bool,
    group_only: bool,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut options = Options::default();
    let mut path = PathBuf::from(".");

    // Parsing des options
    for arg in &args[1..] {
        match arg.as_str() {
            "-a" => options.show_hidden = true,
            "-l" => options.long_format = true,
            "-d" => options.directories_only = true,
            "-t" => options.sort_by_time = true,
            "-r" => options.reverse_order = true,
            "-R" => options.recursive = true,
            "-u" => options.access_time = true,
            "-F" => options.show_type = true,
            "-f" => options.unsorted = true,
            "-g" => {
                options.long_format = true;
                options.group_only = true;
            }
            _ => path = PathBuf::from(arg),
        }
    }

    list_directory(&path, &options);
}

fn list_directory(path: &Path, options: &Options) {
    match fs::read_dir(path) {
        Ok(entries) => {
            let mut entries: Vec<DirEntry> = entries.filter_map(|e| e.ok()).collect();

            if options.sort_by_time {
                entries.sort_by(|a, b| compare_by_time(a, b, options.access_time));
            } else if !options.unsorted {
                entries.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
            }

            if options.reverse_order {
                entries.reverse();
            }

            for entry in entries {
                if !options.show_hidden && entry.file_name().to_string_lossy().starts_with('.') {
                    continue;
                }

                let metadata = match entry.metadata() {
                    Ok(m) => m,
                    Err(_) => continue,
                };

                if options.directories_only && !metadata.is_dir() {
                    continue;
                }

                print_entry(&entry, &metadata, options);

                if options.recursive && metadata.is_dir() {
                    println!("\n{}:", entry.path().display());
                    list_directory(&entry.path(), options);
                }
            }
        }
        Err(e) => eprintln!("ardoxlebg-ls: {}", e),
    }
}

fn compare_by_time(a: &DirEntry, b: &DirEntry, access_time: bool) -> Ordering {
    let a_time = get_time(a.metadata().ok(), access_time);
    let b_time = get_time(b.metadata().ok(), access_time);
    a_time.cmp(&b_time)
}

fn get_time(metadata: Option<Metadata>, access_time: bool) -> SystemTime {
    if let Some(meta) = metadata {
        if access_time {
            SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(meta.atime() as u64)
        } else {
            SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(meta.mtime() as u64)
        }
    } else {
        SystemTime::UNIX_EPOCH
    }
}

fn print_entry(entry: &DirEntry, metadata: &Metadata, options: &Options) {
    if options.long_format {
        print_permissions(metadata);

        print!(" {} ", metadata.nlink());
        if !options.group_only {
            print!("{} ", metadata.uid());
        }
        print!("{} {}", metadata.gid(), metadata.len());

        let time = if options.access_time { metadata.atime() } else { metadata.mtime() };
        let time_str = format_time(time);
        print!(" {} ", time_str);
    }

    print!("{}", entry.file_name().to_string_lossy());

    if options.show_type {
        if metadata.is_dir() {
            print!("/");
        } else if metadata.permissions().mode() & 0o111 != 0 {
            print!("*");
        }
    }

    println!();
}

fn print_permissions(metadata: &Metadata) {
    let mode = metadata.permissions().mode();
    print!("{}", if metadata.is_dir() { "d" } else { "-" });
    print!("{}", if mode & 0o400 != 0 { "r" } else { "-" });
    print!("{}", if mode & 0o200 != 0 { "w" } else { "-" });
    print!("{}", if mode & 0o100 != 0 { "x" } else { "-" });
    print!("{}", if mode & 0o040 != 0 { "r" } else { "-" });
    print!("{}", if mode & 0o020 != 0 { "w" } else { "-" });
    print!("{}", if mode & 0o010 != 0 { "x" } else { "-" });
    print!("{}", if mode & 0o004 != 0 { "r" } else { "-" });
    print!("{}", if mode & 0o002 != 0 { "w" } else { "-" });
    print!("{}", if mode & 0o001 != 0 { "x" } else { "-" });
}

fn format_time(time: i64) -> String {
    let naive_time = chrono::NaiveDateTime::from_timestamp_opt(time, 0).expect("Invalid timestamp");
    naive_time.format("%b %d %H:%M").to_string()
}
