use log::{log, Level};
use paste::paste;

macro_rules! result_log_trait_group {
    ($level:ident) => {
        paste! {
            fn $level(self, message: &str) -> Result<T, E>;

            fn [<$level _ok>](self, message: &str) -> Result<T, E>;

            fn [<$level _err>](self, message: &str) -> Result<T, E>;
        }
    };
}

macro_rules! result_format_trait_group {
    ($level:ident) => {
        paste! {
            fn [<$level _format>]<F>(self, f: F) -> Result<T, E>
            where
                F: FnOnce(&Result<T, E>) -> String;

            fn [<$level _ok_format>]<F>(self, f: F) -> Result<T, E>
            where
                F: FnOnce(&T) -> String;

            fn [<$level _err_format>]<F>(self, f: F) -> Result<T, E>
            where
                F: FnOnce(&E) -> String;
        }
    };
}

pub trait ResultLogger<T, E> {
    fn log(self, level: Level, message: &str) -> Result<T, E>;
    fn log_ok(self, level: Level, message: &str) -> Result<T, E>;
    fn log_err(self, level: Level, message: &str) -> Result<T, E>;

    result_log_trait_group!(trace);
    result_log_trait_group!(debug);
    result_log_trait_group!(info);
    result_log_trait_group!(warn);
    result_log_trait_group!(error);
}

pub trait ResultLogFormatter<T, E> {
    fn log_format<F>(self, level: Level, f: F) -> Result<T, E>
    where
        F: FnOnce(&Result<T, E>) -> String;
    fn log_ok_format<F>(self, level: Level, f: F) -> Result<T, E>
    where
        F: FnOnce(&T) -> String;
    fn log_err_format<F>(self, level: Level, f: F) -> Result<T, E>
    where
        F: FnOnce(&E) -> String;

    result_format_trait_group!(trace);
    result_format_trait_group!(debug);
    result_format_trait_group!(info);
    result_format_trait_group!(warn);
    result_format_trait_group!(error);
}

macro_rules! result_log_impl_group {
    ($level_name:ident, $level:expr) => {
        paste! {
            fn $level_name(self, message: &str) -> Result<T, E> {
                self.log($level, message)
            }

            fn [<$level_name _ok>](self, message: &str) -> Result<T, E> {
                self.log_ok($level, message)
            }

            fn [<$level_name _err>](self, message: &str) -> Result<T, E> {
                self.log_err($level, message)
            }
        }
    };
}

macro_rules! result_format_impl_group {
    ($level_name:ident, $level:expr) => {
        paste! {
            fn [<$level_name _format>]<F>(self, f: F) -> Result<T, E>
            where
                F: FnOnce(&Result<T, E>) -> String,
            {
                self.log_format($level, f)
            }

            fn [<$level_name _ok_format>]<F>(self, f: F) -> Result<T, E>
            where
                F: FnOnce(&T) -> String,
            {
                self.log_ok_format($level, f)
            }

            fn [<$level_name _err_format>]<F>(self, f: F) -> Result<T, E>
            where
                F: FnOnce(&E) -> String,
            {
                self.log_err_format($level, f)
            }
        }
    };
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

    fn log_ok(self, level: Level, message: &str) -> Result<T, E> {
        self.map(|v| {
            log!(level, "{}: {:?}", message, v);
            v
        })
    }

    fn log_err(self, level: Level, message: &str) -> Result<T, E> {
        self.map_err(|e| {
            log!(level, "{}: {:?}", message, e);
            e
        })
    }

    result_log_impl_group!(trace, Level::Trace);
    result_log_impl_group!(debug, Level::Debug);
    result_log_impl_group!(info, Level::Info);
    result_log_impl_group!(warn, Level::Warn);
    result_log_impl_group!(error, Level::Error);
}

impl<T, E> ResultLogFormatter<T, E> for Result<T, E> {
    fn log_format<F>(self, level: Level, f: F) -> Result<T, E>
    where
        F: FnOnce(&Result<T, E>) -> String,
    {
        if log::log_enabled!(level) {
            log!(level, "{}", f(&self));
        }
        self
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

    result_format_impl_group!(trace, Level::Trace);
    result_format_impl_group!(debug, Level::Debug);
    result_format_impl_group!(info, Level::Info);
    result_format_impl_group!(warn, Level::Warn);
    result_format_impl_group!(error, Level::Error);
}
