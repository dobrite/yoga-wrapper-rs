#[repr(C)]
#[derive(Debug)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Verbose,
}
