use std::collections::BTreeMap as Map;
use std::env;

use structopt::StructOpt;
use tagsearch::{
    filter::Filter,
    utility::{get_files, get_tags_for_file},
};

mod budgetitem;
use budgetitem::BudgetItem;

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

    /// Show summary of all tags related to filtered files
    #[structopt(long)]
    all: bool,
}

fn main() -> Result<()> {
    let opts = Opt::from_args();
    let finance_dir = match env::var("FINANCES") {
        Ok(f) => f,
        Err(_) => {
            return Err("Must provide directories, or set FINANCES env var".into());
        }
    };

    let mut tags = opts.tags.iter().map(|x| x.as_str()).collect::<Vec<&str>>();
    if !opts.archive {
        tags.push("!archive");
    }
    let filter = Filter::new(&tags, false);
    let mut tag_map: Map<String, Vec<BudgetItem>> = Map::new();
    let mut matching_files = Vec::new();
    for file in get_files(Some(finance_dir))? {
        let b = BudgetItem::new(file.clone())?;
        if filter.matches(&b.tags) {
            for tag in &b.tags {
                (*tag_map.entry(tag.to_string()).or_insert(Vec::new())).push(b.clone());
            }
            matching_files.push(b);
        }
    }
    let tagmap_related_keywords: Vec<String> = tag_map
        .keys()
        .filter(|keyword| !tags.contains(&keyword.as_str()))
        .map(|x| x.to_owned())
        .collect();

    let total_for_matching: f64 = matching_files.iter().map(|x| x.cost).sum();
    if opts.tags.is_empty() {
        println!("Total: {:.2}", total_for_matching);
    } else {
        println!(
            "Total for `{}`: {:.2}",
            opts.tags.join("+"),
            total_for_matching
        );
    }

    if !opts.all {
        if opts.verbose {
            for item in matching_files {
                println!("\t{}", item);
            }
            println!();
        }
        println!("Related: {}", tagmap_related_keywords.join(" "));
    } else {
        println!("\nBreakdown for matching tags");
        for (key, budgetitems) in tag_map {
            let total: f64 = budgetitems.iter().map(|x| x.cost).sum();
            println!("{} - {:.2}", key, total);
            if opts.verbose {
                for item in budgetitems {
                    println!("\t{}", item);
                }
            }
        }
    }

    Ok(())
}
