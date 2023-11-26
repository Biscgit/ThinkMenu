use tokio::{io, fs};
use tokio::io::AsyncWriteExt;

pub async fn read_file(file_path: &str) -> io::Result<String> {
    let contents = fs::read_to_string(file_path).await?;
    Ok(contents)
}

pub async fn write_file(file_path: &str, contents: &str) -> io::Result<()> {
    let mut file = fs::OpenOptions::new().truncate(true).write(true).open(file_path).await?;

    file.write_all(contents.as_bytes()).await?;
    file.flush().await?;
    Ok(())
}
