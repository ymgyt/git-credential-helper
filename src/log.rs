pub trait Log {
    fn log(&self, message: &str);
}

pub struct StdErrLogger;

impl Log for StdErrLogger {
    fn log(&self, message: &str) {
        eprintln!("git-credential-helper: {}", message);
    }
}

pub struct NopLogger;

impl Log for NopLogger {
    fn log(&self, _message: &str) {}
}
