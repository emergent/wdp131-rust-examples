#[tokio::main]
async fn main() -> std::io::Result<()> {
    let _ = tokio::fs::read("file.txt").await?;
    Ok(())
}
