pub fn crate_version() -> String {
    String::from(format!("Bakery GraphQL server version {version}", version = env!("CARGO_PKG_VERSION")))
}