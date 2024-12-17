use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://www.baidu.com";

    let mut response = reqwest::get(url).await?;
    let content = response.text().await?;
    println!("{}", content);
    Ok(())
}
