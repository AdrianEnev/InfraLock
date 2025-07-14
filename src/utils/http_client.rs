use std::path::Path;
use std::io;

/// Download a file from a URL to a local path asynchronously.
pub async fn download_file(url: &str, dest: &Path) -> io::Result<()> {
    let response = reqwest::get(url).await.map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    let bytes = response.bytes().await.map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    tokio::fs::write(dest, &bytes).await?;
    Ok(())
} 