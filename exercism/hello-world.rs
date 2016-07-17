
pub fn hello(name: Option<&str>) -> String {
    match name {
        Some(s) => format!("Hello, {}!", s.to_string()),
        _ => "Hello, World!".to_string()
    }
}
