use clap::Parser;
use colored::*;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::time::Duration;

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

    #[arg(long)]
    stats: bool,
}
#[derive(Default)]
struct Stats {
    file_count: usize,
    dir_count: usize,
    total_size: u64,
}
fn format_duration(dur: Duration) -> String {
    let micros = dur.as_micros();

    if micros >= 1_000_000 {
        format!("{:.2}s", micros as f64 / 1_000_000.0)
    } else if micros >= 1_000 {
        format!("{:.2}ms", micros as f64 / 1_000.0)
    } else {
        format!("{}μs", micros)
    }
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
    let now = std::time::Instant::now();
    let args = Args::parse();
    let root = &args.path;

    let mut output: Box<dyn Write> = match &args.output {
        Some(path) => Box::new(io::BufWriter::new(fs::File::create(path)?)),
        None => Box::new(io::stdout()),
    };

    writeln!(output, "{}", root.display())?;

    let mut stats = Stats::default();
    let stats_opt = if args.stats { Some(&mut stats) } else { None };

    print_tree(
        &mut output,
        &args.path,
        "".to_string(),
        0,
        args.depth.unwrap_or(usize::MAX),
        args.show_all,
        &args.ignore,
        &args.include,
        stats_opt,
    )?;

    if args.stats {
        use humansize::{DECIMAL, format_size};
        println!(
            "\n📁 {} directories, 📄 {} files\n📦 Total size: {}",
            stats.dir_count,
            stats.file_count,
            format_size(stats.total_size, DECIMAL)
        );
    }
    let elapsed = now.elapsed();
    let human_readable = format_duration(elapsed);

    println!("\n⏱️ Processed in {}", human_readable);
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
    mut stats: Option<&mut Stats>,
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

    let stats_ref = &mut stats;
    for (i, entry) in entries.into_iter().enumerate() {
        let path = entry.path();
        let is_dir = path.is_dir();

        if let Some(s) = stats_ref {
            if path.is_dir() {
                s.dir_count += 1;
            } else if let Ok(meta) = path.metadata() {
                s.file_count += 1;
                s.total_size += meta.len();
            }
        }

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
                stats_ref.as_deref_mut(),
            )?;
        }
    }

    Ok(())
}
