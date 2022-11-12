use downloader::ParseFilename;
use std::{env, fs::File, io::Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let url = args.get(1).expect("<url> is not provided");

    let mut response = downloader::download(url).await?;

    let original_filename = response.get_filename();
    let filename = args.get(2).unwrap_or(&original_filename);
    let mut file = File::create(format!("{}", filename))?;
    while let Some(chunk) = response.chunk().await? {
        file.write_all(&chunk)
            .or(Err(format!("Error while writing to file")))?;
    }

    Ok(())
}
