use warp::Filter;

const WEB_FOLDER: &str = "web-folder/";

#[tokio::main]
async fn main() {
    // GET / root page
    let hello = warp::path::end().map(|| "Hello, World at root!");
    // GET /hi page
    let hi = warp::path!("hi").map(|| "hi");
    // GET sum number + number = sum
    let sum = warp::path!("sum" / u32 / u32).map(|a, b| format!("{} + {} = {}", a, b, a + b));
    // Content-Type: text/html
    let content = warp::fs::dir(WEB_FOLDER);
    // Route switch
    let routes = warp::get().and(hello.or(hi).or(sum).or(content));
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
