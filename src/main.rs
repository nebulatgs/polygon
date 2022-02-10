mod lexer;
mod tokens;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Sync + Send>> {
    let mut sources = Vec::<String>::new();
    for arg in std::env::args().skip(1) {
        let source = tokio::fs::read_to_string(arg).await?;
        sources.push(source);
    }
    for source in sources.into_iter() {
        let mut lexer = lexer::Lexer::new(source);
        loop {
            if let Some(token) = lexer.next_token() {
                println!("{:?}", token);
                continue;
            }
            break;
        }
    }
    Ok(())
}