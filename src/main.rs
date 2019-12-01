use std::env;
use std::path::PathBuf;

use shellexpand::tilde;
use structopt::StructOpt;

mod budgetitem;

use budgetitem::BudgetItem;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, StructOpt)]
#[structopt(name = "budget", about = "summarise individual budget files")]
struct Opt {
    /// Directories to summarise
    directories: Vec<String>,

    /// Show individual categories, as well as summary
    #[structopt(short, long)]
    verbose: bool,

    /// Show archive
    #[structopt(short, long)]
    archive: bool,

    /// Show only a specific folder
    #[structopt(short, long)]
    folder: Option<String>,
}

fn process_dir(dir: PathBuf) -> Result<Vec<BudgetItem>> {
    if !dir.is_dir() {
        return Err(format!("{:?} is not a dir", dir).into());
    }
    let mut entries = Vec::new();
    for entry in dir.read_dir()? {
        if let Ok(x) = entry {
            entries.push(BudgetItem::new(x.path())?);
        }
    }
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
    let finance_dir = env::var("FINANCES");
    let root: Vec<String> = if !opts.directories.is_empty() {
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
        return Err("Must provide directories, or set FINANCES env var".into());
    };

    let folder_to_filter = match opts.folder {
        Some(foldername) => {
            if opts.verbose {
                println!("Only showing folder `{}`\n", foldername);
            }
            foldername
        }
        None => String::new(),
    };
    for direc in root {
        let filename = tilde(&direc).to_string();
        let p = PathBuf::from(filename);
        let fname: String = p.file_name().unwrap().to_string_lossy().to_string();
        if folder_to_filter != "" && folder_to_filter != fname {
            continue;
        }

        if fname.contains("archive") && !opts.archive {
            continue;
        }
        let entries = process_dir(p)?;
        let summed: f64 = entries.iter().map(|x| x.cost).sum();
        let titlestring = format!("£{:<10.0} {:20}", summed, fname);
        println!("{}", titlestring);
        if opts.verbose {
            println!("{}", "=".repeat(titlestring.trim().len()));
        }
        if opts.verbose {
            for entry in entries {
                println!("{}", entry);
            }
        }
        if opts.verbose {
            println!();
        }
    }
    Ok(())
}
