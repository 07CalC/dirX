use clap::Parser;
use colored::*;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(default_value = ".")]
    path: PathBuf,

    #[arg(short, long)]
    depth: Option<usize>,

    #[arg(long)]
    show_all: bool,

    #[arg(long, value_name = "NAME")]
    ignore: Vec<String>,

    #[arg(long, value_name = "NAME")]
    include: Vec<String>,

    #[arg(long, value_name = "FILE")]
    output: Option<PathBuf>,
}

fn is_ignored(
    name: &str,
    show_all: bool,
    custom_ignores: &[String],
    force_includes: &[String],
) -> bool {
    if custom_ignores.contains(&name.to_string()) {
        return true;
    }

    if show_all {
        return false;
    }

    let default_ignored = [
        ".git",
        ".hg",
        ".svn",
        ".DS_Store",
        "node_modules",
        "target",
        "dist",
        "build",
        ".cache",
        ".next",
        ".turbo",
        ".vercel",
        ".idea",
        ".vscode",
        "venv",
        "__pycache__",
        ".pytest_cache",
        ".mypy_cache",
        ".tox",
        "out",
        "coverage",
        ".parcel-cache",
    ];

    if default_ignored.contains(&name) {
        !force_includes.contains(&name.to_string())
    } else {
        false
    }
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let root = &args.path;

    let mut output: Box<dyn Write> = match &args.output {
        Some(path) => Box::new(io::BufWriter::new(fs::File::create(path)?)),
        None => Box::new(io::stdout()),
    };

    writeln!(output, "{}", root.display())?;

    print_tree(
        &mut output,
        root,
        "".to_string(),
        0,
        args.depth.unwrap_or(usize::MAX),
        args.show_all,
        &args.ignore,
        &args.include,
    )?;

    Ok(())
}

fn print_tree<W: Write>(
    writer: &mut W,
    path: &Path,
    prefix: String,
    level: usize,
    max_depth: usize,
    show_all: bool,
    custom_ignores: &[String],
    force_includes: &[String],
) -> io::Result<()> {
    if level >= max_depth {
        return Ok(());
    }

    let Ok(read_dir) = fs::read_dir(path) else {
        return Ok(());
    };
    let mut entries: Vec<_> = read_dir
        .filter_map(Result::ok)
        .filter(|e| {
            let name = e.file_name().to_string_lossy().to_string();
            !is_ignored(&name, show_all, custom_ignores, force_includes)
        })
        .collect();

    entries.sort_by_key(|e| e.path());
    let total = entries.len();

    for (i, entry) in entries.into_iter().enumerate() {
        let path = entry.path();
        let is_dir = path.is_dir();

        let file_name = path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("<invalid>");

        let mut display_name = if is_dir {
            file_name.blue().bold()
        } else {
            file_name.normal()
        };

        if path
            .symlink_metadata()
            .map(|m| m.file_type().is_symlink())
            .unwrap_or(false)
        {
            display_name = display_name.yellow().italic();
        }

        let is_last = i == total - 1;
        let branch = if is_last { "└── " } else { "├── " };

        writeln!(writer, "{}{}{}", prefix, branch, display_name)?;

        if is_dir {
            let new_prefix = if is_last {
                format!("{}    ", prefix)
            } else {
                format!("{}│   ", prefix)
            };
            print_tree(
                writer,
                &path,
                new_prefix,
                level + 1,
                max_depth,
                show_all,
                custom_ignores,
                force_includes,
            )?;
        }
    }

    Ok(())
}
