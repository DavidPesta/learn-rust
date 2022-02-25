#[derive(Clone, PartialEq, Debug)]
pub enum LogLevel {
	Info,
	Warning,
	Error,
}

/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
	match level {
		LogLevel::Info => "[INFO]: ".to_owned() + message,
		LogLevel::Warning => "[WARNING]: ".to_owned() + message,
		LogLevel::Error => "[ERROR]: ".to_owned() + message
	}
}

pub fn info(message: &str) -> String {
	log(LogLevel::Info, &message)
}

pub fn warn(message: &str) -> String {
	log(LogLevel::Warning, &message)
}

pub fn error(message: &str) -> String {
	log(LogLevel::Error, &message)
}
