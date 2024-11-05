# ls

## Description
This is my own ls command made in rust. This is absolutely useless and I made it just for fun and to learn rust.
Don't use it as your main ls command lol, it's not even close to the real ls command.

## Installation
Instructions on how to install and set up the project.

```bash
# Clone the repository
git clone https://github.com/levraiardox/ls.git

# Navigate to the project directory
cd ls

# Build the project
cargo build --release
```

## Usage
How to use the project after installation.
The executable will be located in the `target/release` directory as `ls`.

```bash
# Run the project
./ls [OPTIONS] [PATH]
```

These are the available options:
- `-a`: Show hidden files.
- `-l`: Show detailed information about the files.
- `-d`: List directories themselves, not their contents.
- `-t`: Sort by modification time.
- `-r`: Reverse the order of the sort.
- `-R`: List subdirectories recursively.
- `-u`: Use file access time for sorting.
- `-F`: Append indicator (one of */=>@|) to entries.
- `-f`: Do not sort.
- `-g`: Do not list owner.
