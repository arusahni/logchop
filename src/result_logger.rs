use log::{log, Level};
use paste::paste;

macro_rules! result_log_trait_group {
    ($level:ident, $level_name:expr) => {
        paste! {
            #[doc = "Output a "]
            #[doc = $level_name]
            #[doc = " log with the given message. This will append the wrapped value."]
            fn $level(self, message: &str) -> Result<T, E>;

            #[doc = "Output a "]
            #[doc = $level_name]
            #[doc = " log with the given message if `Ok`, appending the wrapped value."]
            fn [<$level _ok>](self, message: &str) -> Result<T, E>;

            #[doc = "Output a "]
            #[doc = $level_name]
            #[doc = " log with the given message if `Err`, appending the wrapped value."]
            fn [<$level _err>](self, message: &str) -> Result<T, E>;
        }
    };

    ($level:tt) => {
        result_log_trait_group!($level, stringify!($level));
    };
}

macro_rules! result_format_trait_group {
    ($level:ident, $level_name:expr) => {
        paste! {
            #[doc = "Output a "]
            #[doc = $level_name]
            #[doc = " log with the return value of the provided closure."]
            fn [<$level _format>]<F>(self, f: F) -> Result<T, E>
            where
                F: FnOnce(&Result<T, E>) -> String;

            #[doc = "Output a "]
            #[doc = $level_name]
            #[doc = " log with the return value of the provided closure if `Ok`."]
            fn [<$level _ok_format>]<F>(self, f: F) -> Result<T, E>
            where
                F: FnOnce(&T) -> String;

            #[doc = "Output a "]
            #[doc = $level_name]
            #[doc = " log with the return value of the provided closure if `Err`."]
            fn [<$level _err_format>]<F>(self, f: F) -> Result<T, E>
            where
                F: FnOnce(&E) -> String;
        }
    };

    ($level:tt) => {
        result_format_trait_group!($level, stringify!($level));
    };
}

/// Augment [`Result`] types with log methods that can print debug representations of wrapped
/// values. If the wrapped `Ok` or `Err` types do not implement `Debug`, use [`ResultLogFormatter`]
/// instead.
///
/// This trait defines methods prefixed by the log level, as well as a general one that accepts a
/// log level parameter.
///
/// * `<log_level>(message)` appends the option to the message if `Ok` or `Err`
/// * `<log_level>_ok(message)` appends the wrapped value to the message if `Ok`
/// * `<log_level>_err(message)` appends the wrapped value to the message if `Err`
///
/// [`Result`]: https://doc.rust-lang.org/std/result/enum.Result.html
/// [`ResultLogFormatter`]: crate::result_logger::ResultLogFormatter
pub trait ResultLogger<T, E> {
    /// Output a log message of the provided severity if `Ok` or `Err`. The wrapped value will be
    /// appended.
    fn log(self, level: Level, message: &str) -> Result<T, E>;

    /// Output a log message of the provided severity if `Ok`. The wrapped value will be appended.
    fn log_ok(self, level: Level, message: &str) -> Result<T, E>;

    /// Output a log message of the provided severity if `Err`. The wrapped value will be appended.
    fn log_err(self, level: Level, message: &str) -> Result<T, E>;

    result_log_trait_group!(trace);
    result_log_trait_group!(debug);
    result_log_trait_group!(info);
    result_log_trait_group!(warn);
    result_log_trait_group!(error);
}

/// Augment [`Result`] types with log methods that can print abitrary representations of wrapped
/// values. If the type being logged implements `Debug`, you may be able to use [`ResultLogger`]
/// instead.
///
/// The callsite is responsible for defining the format strategy. The closure is only evaluated
/// when necessary.
///
/// This trait defines methods prefixed by the log level, as well as a general one that accepts a
/// log level parameter.
///
/// * `<log_level>_format(FnOnce)` prints the return value of the closure if `Ok` or `Err`
/// * `<log_level>_format_ok(FnOnce)` prints the return value of the closure if `Ok`
/// * `<log_level>_format_err(FnOnce)` prints the return value of the closure if `Err`
///
/// [`Result`]: https://doc.rust-lang.org/std/result/enum.Result.html
/// [`ResultLogger`]: crate::result_logger::ResultLogger
pub trait ResultLogFormatter<T, E> {
    /// Output a log message of the provided severity if `Ok` or `Err`. The message is
    /// determined by the return value of the supplied closure.
    fn log_format<F>(self, level: Level, f: F) -> Result<T, E>
    where
        F: FnOnce(&Result<T, E>) -> String;

    /// Output a log message of the provided severity if `Ok`. The message is determined by the
    /// return value of the supplied closure.
    fn log_ok_format<F>(self, level: Level, f: F) -> Result<T, E>
    where
        F: FnOnce(&T) -> String;

    /// Output a log message of the provided severity if `Err`. The message is determined by the
    /// return value of the supplied closure.
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
