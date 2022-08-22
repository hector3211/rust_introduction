extern crate cli_news;
use self::cli_news::{get_articles, Articles};
use colour::{dark_cyan, dark_green};
fn render_colored_articles(articles: &Articles) {
    for article in &articles.articles {
        dark_cyan!("> {}\n", article.title);
        dark_green!("- {}\n\n", article.url);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let json_arts = get_articles().await?;
    render_colored_articles(&json_arts);
    Ok(())
}
