use std::error::Error;

mod ai;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    ai::prompt_loop().await?;
    Ok(())
}
