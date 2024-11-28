use clap::{Parser, ValueEnum};
use retch::{retcher, Browser as RetchBrowser};

#[derive(Parser, Debug, Clone, Copy, ValueEnum)]
enum Browser {
    Chrome,
    Firefox,
    Retch
}

#[derive(Parser, Debug, Clone, Copy, ValueEnum)]
enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    OPTIONS,
    TRACE
}

/// CLI interface for the retch library.
/// Something like CURL for libcurl, for making impersonated HTTP(2) requests.
#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct CliArgs {
    /// Method to use for the request.
    #[arg(short='X', long, default_value = "get")]
    method: Method,

    /// HTTP headers to add to the request.
    #[arg(short='H', long)]
    headers: Vec<String>,
    
    /// What browser to use for the request.
    #[arg(short='A', long, default_value = "retch")]
    impersonate: Browser,

    /// If set, retch will ignore TLS errors.
    #[arg(short='k', long, action)]
    ignore_tls_errors: bool,
    
    /// If set, retch will fallback to vanilla HTTP if the impersonated browser fails.
    #[arg(short='f', long, action)]
    fallback: bool,

    /// URL of the request to make
    url: String,
}

#[tokio::main]
async fn main() {
    let args = CliArgs::parse();

    let mut client = retcher::Retcher::builder()
        .with_ignore_tls_errors(args.ignore_tls_errors)
        .with_fallback_to_vanilla(args.fallback);

    client = match args.impersonate {
        Browser::Chrome => client.with_browser(RetchBrowser::Chrome),
        Browser::Firefox => client.with_browser(RetchBrowser::Firefox),
        Browser::Retch => client
    };

    let client = client.build();

    let response = match args.method {
        Method::GET => client.get(args.url, None).await.unwrap(),
        Method::POST => client.post(args.url, None, None).await.unwrap(),
        Method::PUT => client.put(args.url, None, None).await.unwrap(),
        Method::DELETE => client.delete(args.url, None).await.unwrap(),
        Method::PATCH => client.patch(args.url, None, None).await.unwrap(),
        Method::HEAD => client.head(args.url, None).await.unwrap(),
        Method::OPTIONS => client.options(args.url, None).await.unwrap(),
        Method::TRACE => client.trace(args.url, None).await.unwrap(),
    };

    print!("{}", response.text().await.unwrap());
}