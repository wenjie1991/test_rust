use extendr_api::prelude::*;
use openssl::ssl::{SslConnector, SslMethod};

#[extendr]
fn check_ssl() -> i32{
    let _ = SslConnector::builder(SslMethod::tls()).unwrap();
    1 + 1
}

// Macro to generate exports
extendr_module! {
    mod test;
    fn check_ssl;
}
