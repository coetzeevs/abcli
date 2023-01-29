use color_eyre::Report;
use dotenv::dotenv;
use tracing_subscriber::EnvFilter;


pub fn setup() -> Result<(), Report> {
    // Settings for development
    if std::env::var("RUST_LIB_BACKTRACE").is_err() {
        std::env::set_var("RUST_LIB_BACKTRACE", "1")
    }
    color_eyre::install()?;

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }
    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // load env
    dotenv().ok();

    // use tracing::info;
    // use crate::app::secrets::get_secret;
    // test secret implementation
    // let mut test = std::collections::HashMap::new();
    // test.insert(String::from("db:username"), String::from(""));
    // test.insert(String::from("db:password"), String::from(""));
    // let sec = get_secret(&mut test)?;
    // info!("{sec:#?}");
    
    Ok(())
}
