use std::env;

use structopt::StructOpt;

mod budgetitem;

use budgetitem::BudgetItem;
use tagsearch::{
    filter::Filter,
    utility::{get_files, get_tags_for_file},
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, StructOpt)]
#[structopt(name = "budget", about = "summarise individual budget files")]
struct Opt {
    /// Show only entries matching tags
    tags: Vec<String>,

    /// Show archive
    #[structopt(short, long)]
    archive: bool,

    /// Show files matching each tags
    #[structopt(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let opts = Opt::from_args();
    let finance_dir = match env::var("FINANCES") {
        Ok(f) => f,
        Err(_) => {
            return Err("Must provide directories, or set FINANCES env var".into());
        }
    };

    let tags = opts.tags.iter().map(|x| x.as_str()).collect::<Vec<&str>>();
    let filter = Filter::new(&tags, false);
    for file in get_files(Some(finance_dir))? {
        let b = BudgetItem::new(file)?;
        if filter.matches(&b.tags) {
            println!("{}", b);
        }
    }

    Ok(())
}

