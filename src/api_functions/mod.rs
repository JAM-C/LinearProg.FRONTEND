mod log_in;

pub use log_in::log_in;

pub fn api_url(url: &str) -> String
    { "localhost:8000/api/".to_owned() + url }
