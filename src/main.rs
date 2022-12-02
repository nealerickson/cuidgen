use clap::Parser;

/// Simple program to generate CUIDs
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// number of CUIDs to generate
    #[arg(short, long, default_value_t = 1)]
    count: u8,

    /// prefix of the CUID
    #[arg(short, long)]
    prefix: Option<String>,

    /// generate a slug instead of a full CUID
    #[arg(short, long)]
    slug: bool,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!(
            "{}{}",
            args.prefix.as_deref().unwrap_or_default(),
            if args.slug {
                cuid::slug().unwrap()
            } else {
                cuid::cuid().unwrap()
            }
        )
    }
}
