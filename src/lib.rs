use log::{log, Level};

pub trait OptionLogger<T> {
    fn log(self, level: Level, message: &str) -> Option<T>;
    fn log_format<F>(self, level: Level, f: F) -> Option<T>
    where
        F: FnOnce(&Option<T>) -> String;
    fn log_some(self, level: Level, message: &str) -> Option<T>;
    fn log_some_format<F>(self, level: Level, f: F) -> Option<T>
    where
        F: FnOnce(&T) -> String;
    fn log_none(self, level: Level, message: &str) -> Option<T>;
    fn log_none_format<F>(self, level: Level, f: F) -> Option<T>
    where
        F: FnOnce() -> String;

    fn trace(self, message: &str) -> Option<T>;
    fn trace_format<F>(self, f: F) -> Option<T>
    where
        F: FnOnce(&Option<T>) -> String;
    fn trace_some(self, message: &str) -> Option<T>;
    fn trace_some_format<F>(self, f: F) -> Option<T>
    where
        F: FnOnce(&T) -> String;
    fn trace_none(self, message: &str) -> Option<T>;
    fn trace_none_format<F>(self, f: F) -> Option<T>
    where
        F: FnOnce() -> String;

    fn debug(self, message: &str) -> Option<T>;
    fn debug_format<F>(self, f: F) -> Option<T>
    where
        F: FnOnce(&Option<T>) -> String;
    fn debug_some(self, message: &str) -> Option<T>;
    fn debug_some_format<F>(self, f: F) -> Option<T>
    where
        F: FnOnce(&T) -> String;
    fn debug_none(self, message: &str) -> Option<T>;
    fn debug_none_format<F>(self, f: F) -> Option<T>
    where
        F: FnOnce() -> String;

    fn info(self, message: &str) -> Option<T>;
    fn info_format<F>(self, f: F) -> Option<T>
    where
        F: FnOnce(&Option<T>) -> String;
    fn info_some(self, message: &str) -> Option<T>;
    fn info_some_format<F>(self, f: F) -> Option<T>
    where
        F: FnOnce(&T) -> String;
    fn info_none(self, message: &str) -> Option<T>;
    fn info_none_format<F>(self, f: F) -> Option<T>
    where
        F: FnOnce() -> String;

    fn warn(self, message: &str) -> Option<T>;
    fn warn_format<F>(self, f: F) -> Option<T>
    where
        F: FnOnce(&Option<T>) -> String;
    fn warn_some(self, message: &str) -> Option<T>;
    fn warn_some_format<F>(self, f: F) -> Option<T>
    where
        F: FnOnce(&T) -> String;
    fn warn_none(self, message: &str) -> Option<T>;
    fn warn_none_format<F>(self, f: F) -> Option<T>
    where
        F: FnOnce() -> String;

    fn error(self, message: &str) -> Option<T>;
    fn error_format<F>(self, f: F) -> Option<T>
    where
        F: FnOnce(&Option<T>) -> String;
    fn error_some(self, message: &str) -> Option<T>;
    fn error_some_format<F>(self, f: F) -> Option<T>
    where
        F: FnOnce(&T) -> String;
    fn error_none(self, message: &str) -> Option<T>;
    fn error_none_format<F>(self, f: F) -> Option<T>
    where
        F: FnOnce() -> String;
}

pub trait ResultLogger<T, E> {
    fn log(self, level: Level, message: &str) -> Result<T, E>;
    fn log_format<F>(self, level: Level, f: F) -> Result<T, E>
    where
        F: FnOnce(&Result<T, E>) -> String;
    fn log_ok(self, level: Level, message: &str) -> Result<T, E>;
    fn log_ok_format<F>(self, level: Level, f: F) -> Result<T, E>
    where
        F: FnOnce(&T) -> String;
    fn log_err(self, level: Level, message: &str) -> Result<T, E>;
    fn log_err_format<F>(self, level: Level, f: F) -> Result<T, E>
    where
        F: FnOnce(&E) -> String;

    fn trace(self, message: &str) -> Result<T, E>;
    fn trace_format<F>(self, f: F) -> Result<T, E>
    where
        F: FnOnce(&Result<T, E>) -> String;
    fn trace_ok(self, message: &str) -> Result<T, E>;
    fn trace_ok_format<F>(self, f: F) -> Result<T, E>
    where
        F: FnOnce(&T) -> String;
    fn trace_err(self, message: &str) -> Result<T, E>;
    fn trace_err_format<F>(self, f: F) -> Result<T, E>
    where
        F: FnOnce(&E) -> String;

    fn debug(self, message: &str) -> Result<T, E>;
    fn debug_format<F>(self, f: F) -> Result<T, E>
    where
        F: FnOnce(&Result<T, E>) -> String;
    fn debug_ok(self, message: &str) -> Result<T, E>;
    fn debug_ok_format<F>(self, f: F) -> Result<T, E>
    where
        F: FnOnce(&T) -> String;
    fn debug_err(self, message: &str) -> Result<T, E>;
    fn debug_err_format<F>(self, f: F) -> Result<T, E>
    where
        F: FnOnce(&E) -> String;

    fn info(self, message: &str) -> Result<T, E>;
    fn info_format<F>(self, f: F) -> Result<T, E>
    where
        F: FnOnce(&Result<T, E>) -> String;
    fn info_ok(self, message: &str) -> Result<T, E>;
    fn info_ok_format<F>(self, f: F) -> Result<T, E>
    where
        F: FnOnce(&T) -> String;
    fn info_err(self, message: &str) -> Result<T, E>;
    fn info_err_format<F>(self, f: F) -> Result<T, E>
    where
        F: FnOnce(&E) -> String;

    fn warn(self, message: &str) -> Result<T, E>;
    fn warn_format<F>(self, f: F) -> Result<T, E>
    where
        F: FnOnce(&Result<T, E>) -> String;
    fn warn_ok(self, message: &str) -> Result<T, E>;
    fn warn_ok_format<F>(self, f: F) -> Result<T, E>
    where
        F: FnOnce(&T) -> String;
    fn warn_err(self, message: &str) -> Result<T, E>;
    fn warn_err_format<F>(self, f: F) -> Result<T, E>
    where
        F: FnOnce(&E) -> String;

    fn error(self, message: &str) -> Result<T, E>;
    fn error_format<F>(self, f: F) -> Result<T, E>
    where
        F: FnOnce(&Result<T, E>) -> String;
    fn error_ok(self, message: &str) -> Result<T, E>;
    fn error_ok_format<F>(self, f: F) -> Result<T, E>
    where
        F: FnOnce(&T) -> String;
    fn error_err(self, message: &str) -> Result<T, E>;
    fn error_err_format<F>(self, f: F) -> Result<T, E>
    where
        F: FnOnce(&E) -> String;
}

impl<T> OptionLogger<T> for Option<T>
where
    T: std::fmt::Debug,
{
    fn log(self, level: Level, message: &str) -> Option<T> {
        if self.is_some() {
            self.log_some(level, message)
        } else {
            self.log_none(level, message)
        }
    }

    fn log_format<F>(self, level: Level, f: F) -> Option<T>
    where
        F: FnOnce(&Option<T>) -> String,
    {
        if log::log_enabled!(level) {
            log!(level, "{}", f(&self));
        }
        self
    }

    fn log_some(self, level: Level, message: &str) -> Option<T> {
        self.map(|v| {
            log!(level, "{}: {:?}", message, v);
            v
        })
    }

    fn log_some_format<F>(self, level: Level, f: F) -> Option<T>
    where
        F: FnOnce(&T) -> String,
    {
        if log::log_enabled!(level) {
            self.map(|v| {
                log!(level, "{}", f(&v));
                v
            })
        } else {
            self
        }
    }

    fn log_none(self, level: Level, message: &str) -> Option<T> {
        if self.is_none() {
            log!(level, "{}: None", message);
        }
        self
    }

    fn log_none_format<F>(self, level: Level, f: F) -> Option<T>
    where
        F: FnOnce() -> String,
    {
        if log::log_enabled!(level) && self.is_none() {
            log!(level, "{}", f());
        }
        self
    }

    fn trace(self, message: &str) -> Option<T> {
        self.log(Level::Trace, message)
    }

    fn trace_format<F>(self, f: F) -> Option<T>
    where
        F: FnOnce(&Option<T>) -> String,
    {
        self.log_format(Level::Trace, f)
    }

    fn trace_some(self, message: &str) -> Option<T> {
        self.log_some(Level::Trace, message)
    }

    fn trace_some_format<F>(self, f: F) -> Option<T>
    where
        F: FnOnce(&T) -> String,
    {
        self.log_some_format(Level::Trace, f)
    }

    fn trace_none(self, message: &str) -> Option<T> {
        self.log_none(Level::Trace, message)
    }

    fn trace_none_format<F>(self, f: F) -> Option<T>
    where
        F: FnOnce() -> String,
    {
        self.log_none_format(Level::Trace, f)
    }

    fn debug(self, message: &str) -> Option<T> {
        self.log(Level::Debug, message)
    }

    fn debug_format<F>(self, f: F) -> Option<T>
    where
        F: FnOnce(&Option<T>) -> String,
    {
        self.log_format(Level::Debug, f)
    }

    fn debug_some(self, message: &str) -> Option<T> {
        self.log_some(Level::Debug, message)
    }

    fn debug_some_format<F>(self, f: F) -> Option<T>
    where
        F: FnOnce(&T) -> String,
    {
        self.log_some_format(Level::Debug, f)
    }

    fn debug_none(self, message: &str) -> Option<T> {
        self.log_none(Level::Debug, message)
    }

    fn debug_none_format<F>(self, f: F) -> Option<T>
    where
        F: FnOnce() -> String,
    {
        self.log_none_format(Level::Debug, f)
    }

    fn info(self, message: &str) -> Option<T> {
        self.log(Level::Info, message)
    }

    fn info_format<F>(self, f: F) -> Option<T>
    where
        F: FnOnce(&Option<T>) -> String,
    {
        self.log_format(Level::Info, f)
    }

    fn info_some(self, message: &str) -> Option<T> {
        self.log_some(Level::Info, message)
    }

    fn info_some_format<F>(self, f: F) -> Option<T>
    where
        F: FnOnce(&T) -> String,
    {
        self.log_some_format(Level::Info, f)
    }

    fn info_none(self, message: &str) -> Option<T> {
        self.log_none(Level::Info, message)
    }

    fn info_none_format<F>(self, f: F) -> Option<T>
    where
        F: FnOnce() -> String,
    {
        self.log_none_format(Level::Info, f)
    }

    fn warn(self, message: &str) -> Option<T> {
        self.log(Level::Warn, message)
    }

    fn warn_format<F>(self, f: F) -> Option<T>
    where
        F: FnOnce(&Option<T>) -> String,
    {
        self.log_format(Level::Warn, f)
    }

    fn warn_some(self, message: &str) -> Option<T> {
        self.log_some(Level::Warn, message)
    }

    fn warn_some_format<F>(self, f: F) -> Option<T>
    where
        F: FnOnce(&T) -> String,
    {
        self.log_some_format(Level::Warn, f)
    }

    fn warn_none(self, message: &str) -> Option<T> {
        self.log_none(Level::Warn, message)
    }

    fn warn_none_format<F>(self, f: F) -> Option<T>
    where
        F: FnOnce() -> String,
    {
        self.log_none_format(Level::Warn, f)
    }

    fn error(self, message: &str) -> Option<T> {
        self.log(Level::Error, message)
    }

    fn error_format<F>(self, f: F) -> Option<T>
    where
        F: FnOnce(&Option<T>) -> String,
    {
        self.log_format(Level::Error, f)
    }

    fn error_some(self, message: &str) -> Option<T> {
        self.log_some(Level::Error, message)
    }

    fn error_some_format<F>(self, f: F) -> Option<T>
    where
        F: FnOnce(&T) -> String,
    {
        self.log_some_format(Level::Error, f)
    }

    fn error_none(self, message: &str) -> Option<T> {
        self.log_none(Level::Error, message)
    }

    fn error_none_format<F>(self, f: F) -> Option<T>
    where
        F: FnOnce() -> String,
    {
        self.log_none_format(Level::Error, f)
    }
}

impl<T, E> ResultLogger<T, E> for Result<T, E>
where
    T: std::fmt::Debug,
    E: std::fmt::Debug,
{
    fn log(self, level: Level, message: &str) -> Result<T, E> {
        if self.is_ok() {
            self.log_ok(level, message)
        } else {
            self.log_err(level, message)
        }
    }

    fn log_format<F>(self, level: Level, f: F) -> Result<T, E>
    where
        F: FnOnce(&Result<T, E>) -> String,
    {
        if log::log_enabled!(level) {
            log!(level, "{}", f(&self));
        }
        self
    }

    fn log_ok(self, level: Level, message: &str) -> Result<T, E> {
        self.map(|v| {
            log!(level, "{}: {:?}", message, v);
            v
        })
    }

    fn log_ok_format<F>(self, level: Level, f: F) -> Result<T, E>
    where
        F: FnOnce(&T) -> String,
    {
        if log::log_enabled!(level) {
            self.map(|res| {
                log!(level, "{}", f(&res));
                res
            })
        } else {
            self
        }
    }

    fn log_err(self, level: Level, message: &str) -> Result<T, E> {
        self.map_err(|e| {
            log!(level, "{}: {:?}", message, e);
            e
        })
    }

    fn log_err_format<F>(self, level: Level, f: F) -> Result<T, E>
    where
        F: FnOnce(&E) -> String,
    {
        if log::log_enabled!(level) {
            self.map_err(|err| {
                log!(level, "{}", f(&err));
                err
            })
        } else {
            self
        }
    }

    fn trace(self, message: &str) -> Result<T, E> {
        self.log(Level::Trace, message)
    }

    fn trace_format<F>(self, f: F) -> Result<T, E>
    where
        F: FnOnce(&Result<T, E>) -> String,
    {
        self.log_format(Level::Trace, f)
    }

    fn trace_ok(self, message: &str) -> Result<T, E> {
        self.log_ok(Level::Trace, message)
    }

    fn trace_ok_format<F>(self, f: F) -> Result<T, E>
    where
        F: FnOnce(&T) -> String,
    {
        self.log_ok_format(Level::Trace, f)
    }

    fn trace_err(self, message: &str) -> Result<T, E> {
        self.log_err(Level::Trace, message)
    }

    fn trace_err_format<F>(self, f: F) -> Result<T, E>
    where
        F: FnOnce(&E) -> String,
    {
        self.log_err_format(Level::Trace, f)
    }

    fn debug(self, message: &str) -> Result<T, E> {
        self.log(Level::Debug, message)
    }

    fn debug_format<F>(self, f: F) -> Result<T, E>
    where
        F: FnOnce(&Result<T, E>) -> String,
    {
        self.log_format(Level::Debug, f)
    }

    fn debug_ok(self, message: &str) -> Result<T, E> {
        self.log_ok(Level::Debug, message)
    }

    fn debug_ok_format<F>(self, f: F) -> Result<T, E>
    where
        F: FnOnce(&T) -> String,
    {
        self.log_ok_format(Level::Debug, f)
    }

    fn debug_err(self, message: &str) -> Result<T, E> {
        self.log_err(Level::Debug, message)
    }

    fn debug_err_format<F>(self, f: F) -> Result<T, E>
    where
        F: FnOnce(&E) -> String,
    {
        self.log_err_format(Level::Debug, f)
    }

    fn info(self, message: &str) -> Result<T, E> {
        self.log(Level::Info, message)
    }

    fn info_format<F>(self, f: F) -> Result<T, E>
    where
        F: FnOnce(&Result<T, E>) -> String,
    {
        self.log_format(Level::Info, f)
    }

    fn info_ok(self, message: &str) -> Result<T, E> {
        self.log_ok(Level::Info, message)
    }

    fn info_ok_format<F>(self, f: F) -> Result<T, E>
    where
        F: FnOnce(&T) -> String,
    {
        self.log_ok_format(Level::Info, f)
    }

    fn info_err(self, message: &str) -> Result<T, E> {
        self.log_err(Level::Info, message)
    }

    fn info_err_format<F>(self, f: F) -> Result<T, E>
    where
        F: FnOnce(&E) -> String,
    {
        self.log_err_format(Level::Info, f)
    }

    fn warn(self, message: &str) -> Result<T, E> {
        self.log(Level::Warn, message)
    }

    fn warn_format<F>(self, f: F) -> Result<T, E>
    where
        F: FnOnce(&Result<T, E>) -> String,
    {
        self.log_format(Level::Warn, f)
    }

    fn warn_ok(self, message: &str) -> Result<T, E> {
        self.log_ok(Level::Warn, message)
    }

    fn warn_ok_format<F>(self, f: F) -> Result<T, E>
    where
        F: FnOnce(&T) -> String,
    {
        self.log_ok_format(Level::Warn, f)
    }

    fn warn_err(self, message: &str) -> Result<T, E> {
        self.log_err(Level::Warn, message)
    }

    fn warn_err_format<F>(self, f: F) -> Result<T, E>
    where
        F: FnOnce(&E) -> String,
    {
        self.log_err_format(Level::Warn, f)
    }

    fn error(self, message: &str) -> Result<T, E> {
        self.log(Level::Error, message)
    }

    fn error_format<F>(self, f: F) -> Result<T, E>
    where
        F: FnOnce(&Result<T, E>) -> String,
    {
        self.log_format(Level::Error, f)
    }

    fn error_ok(self, message: &str) -> Result<T, E> {
        self.log_ok(Level::Error, message)
    }

    fn error_ok_format<F>(self, f: F) -> Result<T, E>
    where
        F: FnOnce(&T) -> String,
    {
        self.log_ok_format(Level::Error, f)
    }

    fn error_err(self, message: &str) -> Result<T, E> {
        self.log_err(Level::Error, message)
    }

    fn error_err_format<F>(self, f: F) -> Result<T, E>
    where
        F: FnOnce(&E) -> String,
    {
        self.log_err_format(Level::Error, f)
    }
}

// One day concat_idents will be in stable, and I'll be a happy boy
//  ... one day.
