/*
use error_chain::error_chain;
error_chain! {
     foreign_links {
         Io(std::io::Error);
         HttpRequest(reqwest::Error);
     }
}

#[tokio::main]
async fn main() -> Result<()> {
    let seed = "https://en.wiktionary.org/wiki/seed";
    let response = reqwest::get(seed).await?;
    let _content =  response.text().await?;

    Ok(())
}
*/
use wiktionary_crawler::crawler::Crawler;

fn main() -> std::io::Result<()> {
   let _  = Crawler::new();
   Ok(())
}
