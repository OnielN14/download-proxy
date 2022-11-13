use clap::Parser;
use downloader::ParseFilename;
use std::{fs::File, io::Write};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_parser = url_required)]
    url: String,

    #[arg(short, long)]
    filename: Option<String>,
}

fn url_required(s: &str) -> Result<String, String> {
    let check_value = Some(s);
    if let Some(v) = check_value {
        Ok(v.to_string())
    } else {
        Err(format!("<url> is not provided"))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli_args = Args::parse();

    let mut response = downloader::download(&cli_args.url).await?;

    let original_filename = response.get_filename();
    let filename = cli_args.filename.unwrap_or(original_filename);
    let mut file = File::create(format!("{}", filename))?;
    while let Some(chunk) = response.chunk().await? {
        file.write_all(&chunk)
            .or(Err(format!("Error while writing to file")))?;
    }

    Ok(())
}
