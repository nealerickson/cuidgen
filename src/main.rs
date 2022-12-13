use clap::Parser;

/// Simple program to generate CUIDs
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// number of CUIDs to generate
    #[arg(id = "NUM", short = 'N', long = "num-cuids", default_value_t = 1)]
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
    let prefix = args.prefix.as_deref().unwrap_or_default();

    for _ in 0..args.count {
        let result = if args.slug {
            cuid::slug()
        } else {
            cuid::cuid()
        };

        let cuid_value = result.unwrap_or_else(|error| {
            panic!("Problem generating CUID value: {:?}", error);
        });

        println!("{}{}", prefix, cuid_value,)
    }
}
