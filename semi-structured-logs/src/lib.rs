// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

/// various log levels
#[derive(Clone, PartialEq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
    Debug
}
/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    match level {
        LogLevel::Error => error(message), 
        LogLevel::Info => info(message),
        LogLevel::Warning => warn(message),
        LogLevel::Debug => debug(message)
    }
}
pub fn info(message: &str) -> String {
    let mut label = "[INFO]: ".to_owned();
    label.push_str(message);
    label
}
pub fn warn(message: &str) -> String {
    let mut label = "[WARNING]: ".to_owned();
    label.push_str(message);
    label
}
pub fn error(message: &str) -> String {
    let mut label = "[ERROR]: ".to_owned();
    label.push_str(message);
    label
}

pub fn debug(message: &str) -> String {
    let mut label = "[DEBUG]: ".to_owned();
    label.push_str(message);
    label
}
