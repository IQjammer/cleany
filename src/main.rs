use clap::Parser;
use jwalk::WalkDir;

#[derive(Parser, Debug)]
struct Args {
    dir: String,
}

#[derive(Debug)]
struct FileInfo {
    size: u64,
    name: String,
}

fn format_size(bytes: u64) -> String {
    const KIB: f64 = 1024.0;
    const MIB: f64 = KIB * 1024.0;
    const GIB: f64 = MIB * 1024.0;
    const TIB: f64 = GIB * 1024.0;

    let b = bytes as f64;

    if b >= TIB {
        format!("{:.2} T", b / TIB)
    } else if b >= GIB {
        format!("{:.2} G", b / GIB)
    } else if b >= MIB {
        format!("{:.2} Mb", b / MIB)
    } else if b >= KIB {
        format!("{:.2} Kb", b / KIB)
    } else {
        format!("{bytes} B")
    }
}

fn parse(args: &Args) {
    const LIMIT: usize = 10;

    let mut top: Vec<FileInfo> = Vec::with_capacity(LIMIT);

    for entry in WalkDir::new(&args.dir).into_iter().filter_map(|e| e.ok()) {
        if !entry.file_type.is_file() {
            continue;
        }

        let Ok(meta) = entry.metadata() else { continue };

        let size = meta.len();

        if top.len() < LIMIT {
            top.push(FileInfo {
                size,
                name: entry.file_name().to_string_lossy().to_string(),
            });
            top.sort_unstable_by_key(|f| f.size);
        } else if size > top[0].size {
            top[0] = FileInfo {
                size,
                name: entry.file_name().to_string_lossy().to_string(),
            };
            top.sort_unstable_by_key(|f| f.size);
        }
    }

    top.reverse();

    for file in top {
        println!("{} {}", format_size(file.size), file.name);
    }
}

fn main() {
    let args = Args::parse();
    parse(&args);
}
