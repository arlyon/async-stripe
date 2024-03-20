fn main() {
    #[cfg(not(any(
        feature = "runtime-tokio-hyper",
        feature = "runtime-tokio-hyper-rustls",
        feature = "runtime-tokio-hyper-rustls-webpki",
        feature = "runtime-blocking",
        feature = "runtime-blocking-rustls",
        feature = "runtime-blocking-rustls-webpki",
        feature = "runtime-async-std-surf",
    )))]
    compile_error!(
        r"one of the following runtime features must be enabled:
        [
            'runtime-tokio-hyper',
            'runtime-tokio-hyper-rustls',
            'runtime-tokio-hyper-rustls-webpki',
            'runtime-blocking',
            'runtime-blocking-rustls',
            'runtime-blocking-rustls-webpki',
            'runtime-async-std-surf'
        ]"
    );
}
