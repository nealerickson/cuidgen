use clap::Parser;
// use cuid;

/// Simple program to generate CUIDs
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Prefix of the CUID
    #[arg(short, long)]
    prefix: Option<String>,

    #[arg(short, long)]
    slug: bool,

    /// Number of CUIDs to generate
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    // println!("{}", args.prefix.as_deref().unwrap_or_default());

    // let prefix = if let Some(pre) = args.prefix.as_deref() {
    //     pre.to_string()
    // } else {
    //     "".to_string()
    // };

    for _ in 0..args.count {
        let cuid_value = if args.slug {
            cuid::slug().unwrap()
        } else {
            cuid::cuid().unwrap()
        };
        println!(
            "{}{}",
            args.prefix.as_deref().unwrap_or_default(),
            cuid_value
        )
    }
}
