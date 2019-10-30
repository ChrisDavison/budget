use std::env;
use std::fmt;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use shellexpand::tilde;
use structopt::StructOpt;

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

struct BudgetItem {
    name: String,
    cost: f64,
    date: Option<String>,
}

impl BudgetItem {
    fn new(filepath: PathBuf) -> Result<BudgetItem> {
        if !filepath.is_file() {
            return Err(format!("{:?} is not a file", filepath).into());
        }
        let mut name = String::new();
        let mut cost = 0.0;
        let mut date: Option<String> = None;
        let f = std::fs::File::open(filepath)?;
        let reader = BufReader::new(f);
        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split(": ").collect();
            if parts[0] == "name" {
                name = parts[1].into();
            } else if parts[0] == "cost" {
                cost = parts[1].parse()?;
            } else if parts[0] == "date" {
                date = Some(parts[1].into());
            }
        }
        Ok(BudgetItem { name, cost, date })
    }
}

impl fmt::Display for BudgetItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let date = match &self.date {
            Some(s) => format!(" ({})", s),
            None => "".to_string(),
        };
        write!(f, "{:8} -- {}{}", self.cost, self.name, date)
    }
}

fn process_dir(dir: PathBuf) -> Result<Vec<BudgetItem>> {
    if !dir.is_dir() {
        return Err(format!("{:?} is not a dir", dir).into());
    }
    let mut entries: Vec<BudgetItem> = dir
        .read_dir()?
        .filter_map(|x| x.ok())
        .filter(|x| x.path().is_file())
        .map(|x| BudgetItem::new(x.path()))
        .filter_map(|x| x.ok())
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
    println!("{:?}", opts);
    let args_dirs: Vec<String> = env::args().skip(1).collect();
    let finance_dir = env::var("FINANCES");
    let mut root: Vec<String> = if !opts.directories.is_empty() {
        opts.directories
    } else if finance_dir.is_ok() {
        let mut v = Vec::new();
        for entry in std::fs::read_dir(finance_dir.unwrap())? {
            if let Ok(entry) = entry {
                if entry.path().is_dir() {
                    v.push(entry.path().to_str().unwrap().to_string());
                }
            }
        }
        v
    } else {
        return Err("Must provide directories, or set FINANCES env var".into())
    };

    for direc in root {
        let filename = tilde(&direc).to_string();
        let p = PathBuf::from(filename);
        let fname: String = p.file_name().unwrap().to_string_lossy().to_string();
        if fname == "archive" && !opts.archive {
            continue
        }
        let entries = process_dir(p)?;
        let summed: f64 = entries.iter().map(|x| x.cost).sum();
        println!("{} ~ £{}", fname, summed);
        if opts.verbose {
            for entry in entries {
                println!("{}", entry);
            }
        }
    }
    Ok(())
}
