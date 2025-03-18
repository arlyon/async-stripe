#[cfg(not(any(feature = "default-tls", feature = "__rustls")))]
compile_error!("Either default-tls or a rustls feature must be enabled");

#[cfg(feature = "default-tls")]
mod default_tls {
    use hyper_tls::HttpsConnector;
    use hyper_util::client::legacy::connect::HttpConnector;
    pub fn create() -> HttpsConnector<HttpConnector> {
        hyper_tls::HttpsConnector::new()
    }

    pub type Connector = HttpsConnector<HttpConnector>;
}

#[cfg(all(feature = "__rustls", not(feature = "default-tls")))]
mod rustls {
    use hyper_rustls::HttpsConnector;
    use hyper_rustls::HttpsConnectorBuilder;
    use hyper_util::client::legacy::connect::HttpConnector;

    pub fn create() -> HttpsConnector<HttpConnector> {
        let builder = HttpsConnectorBuilder::new();
        #[cfg(feature = "rustls-tls-native")]
        let with_roots = builder.with_native_roots().expect("no valid certs found, you may want to use the rustls-tls-webpki-roots feature instead.");
        #[cfg(feature = "rustls-tls-webpki-roots")]
        let with_roots = builder.with_webpki_roots();
        with_roots.https_or_http().enable_http1().enable_http2().build()
    }

    pub type Connector = HttpsConnector<HttpConnector>;
}

#[cfg(feature = "default-tls")]
pub use default_tls::*;
#[cfg(all(feature = "__rustls", not(feature = "default-tls")))]
pub use rustls::*;
