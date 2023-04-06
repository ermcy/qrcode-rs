mod routes;

#[tokio::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/").get(routes::get_world);
    app.at("/qr/:input").get(routes::get_some_word);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}