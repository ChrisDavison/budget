use std::env;
use std::path::PathBuf;
use std::collections::HashMap;

use shellexpand::tilde;
use structopt::StructOpt;
use glob::glob;

mod budgetitem;

use budgetitem::BudgetItem;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, StructOpt)]
#[structopt(name="budget", about="summarise individual budget files")]
struct Opt {
    /// Directories to summarise
    directories: Vec<String>,

    /// Show individual categories, as well as summary
    #[structopt(short, long)]
    verbose: bool,

    /// Show archive
    #[structopt(short, long)]
    archive: bool,
}

fn process_dir(dir: PathBuf) -> Result<Vec<BudgetItem>> {
    if !dir.is_dir() {
        return Err(format!("{:?} is not a dir", dir).into());
    }
    let mut entries: Vec<BudgetItem> = dir
        .read_dir()?
        .filter_map(|x| x.map(|y| y.path()).ok())
        .filter_map(|x| BudgetItem::new(x).ok())
        .collect();

    entries.sort_by(|x, y| {
        if x.cost - y.cost > 0.0001 {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Less
        }
    });
    entries.reverse();
    Ok(entries)
}

fn main() -> Result<()> {
    let opts = Opt::from_args();
    let finance_dir = env::var("FINANCES")?;

    let mut entries_per_dir: HashMap<String, Vec<Vec<String>>> = HashMap::new();
    for entry in glob(&format!("{}/**/*.txt", finance_dir))? {
        let e = entry.map(|x| x.to_string_lossy().to_string())?;
        let mut rel = e.trim_start_matches(&finance_dir);
        if rel.starts_with(std::path::MAIN_SEPARATOR) {
            rel = &rel[1..];
        }
        let parts: Vec<&str> = rel.split(std::path::MAIN_SEPARATOR).collect();
        let parent = parts[0].to_string();
        let name: Vec<String> = parts[1..].to_vec().iter().map(|x| x.to_string()).collect();
        let mut dirs = entries_per_dir.entry(parent).or_insert(Vec::new());
        (*dirs).push(name);
    }
    for (k, v) in entries_per_dir {
        println!("{}", k);
        for val in v {
            println!("\t{:?}", val);
        }
    }

    // let root: Vec<String> = if !opts.directories.is_empty() {
    //     opts.directories
    // } else if finance_dir.is_ok() {
    //     let mut v = Vec::new();
    //     for entry in std::fs::read_dir(finance_dir.unwrap())? {
    //         if let Ok(entry) = entry {
    //             if entry.path().is_dir() {
    //                 v.push(entry.path().to_str().unwrap().to_string());
    //             }
    //         }
    //     }
    //     v
    // } else {
    //     return Err("Must provide directories, or set FINANCES env var".into())
    // };

    // for direc in root {
    //     let filename = tilde(&direc).to_string();
    //     let p = PathBuf::from(filename);
    //     let fname: String = p.file_name().unwrap().to_string_lossy().to_string();
    //     if fname.contains("archive") && !opts.archive {
    //         continue
    //     }
    //     let entries = process_dir(p)?;
    //     let summed: f64 = entries.iter().map(|x| x.cost).sum();
    //     println!("{:20} ~ £{}", fname, summed);
    //     if opts.verbose {
    //         for entry in entries {
    //             println!("{}", entry);
    //         }
    //     }
    // }
    Ok(())
}
