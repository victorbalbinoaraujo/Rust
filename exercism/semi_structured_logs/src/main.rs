#[derive(Clone, PartialEq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}

pub fn log(level: LogLevel, message: &str) -> String {
    let level_str: String = format!("{:?}", level).to_uppercase();
    format!("[{level_str}]: {message}")
}
pub fn info(message: &str) -> String {
    log(LogLevel::Info, message)
}
pub fn warn(message: &str) -> String {
    log(LogLevel::Warning, message)
}
pub fn error(message: &str) -> String {
    log(LogLevel::Error, message)
}

fn main() {
    let l: String = log(LogLevel::Error, "Stack overflow");
    let i: String = info("Timezone changed");

    println!("{l} \n{i}");
}
