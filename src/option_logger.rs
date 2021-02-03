use log::{log, Level};
use paste::paste;

macro_rules! option_log_trait_group {
    ($level:ident, $level_name:expr) => {
        paste! {
            #[doc = "Output a "]
            #[doc = $level_name]
            #[doc = " log with the given message. This will optionally append the wrapped value if `Some`."]
            fn $level(self, message: &str) -> Option<T>;

            #[doc = "Output a "]
            #[doc = $level_name]
            #[doc = " log with the given message if `Some`, appending the wrapped value."]
            fn [<$level _some>](self, message: &str) -> Option<T>;

            #[doc = "Output a "]
            #[doc = $level_name]
            #[doc = " log with the given message if `None`."]
            fn [<$level _none>](self, message: &str) -> Option<T>;
        }
    };

    ($level:tt) => {
        option_log_trait_group!($level, stringify!($level));
    };
}

macro_rules! option_format_trait_group {
    ($level:ident, $level_name:expr) => {
        paste! {
            #[doc = "Output a "]
            #[doc = $level_name]
            #[doc = " log with the return value of the provided closure."]
            fn [<$level _format>]<F>(self, f: F) -> Option<T>
            where
                F: FnOnce(&Option<T>) -> String;

            #[doc = "Output a "]
            #[doc = $level_name]
            #[doc = " log with the return value of the provided closure if `Some`."]
            fn [<$level _some_format>]<F>(self, f: F) -> Option<T>
            where
                F: FnOnce(&T) -> String;

            #[doc = "Output a "]
            #[doc = $level_name]
            #[doc = " log with the return value of the provided closure if `None`."]
            fn [<$level _none_format>]<F>(self, f: F) -> Option<T>
            where
                F: FnOnce() -> String;
        }
    };

    ($level:tt) => {
        option_format_trait_group!($level, stringify!($level));
    };
}

/// Augment [`Option`] types with log methods that can print `Debug` representations of wrapped
/// values. If the type being logged does not implement `Debug`, use [`OptionLogFormatter`]
/// instead.
///
/// This trait defines methods prefixed by the log level, as well as a general one that accepts a
/// log level parameter.
///
/// * `<log_level>(message)` appends the option to the message if `Some`, or prints the message if `None`
/// * `<log_level>_some(message)` appends the wrapped value to the message if `Some`
/// * `<log_level>_none()` outputs the message if `None`
///
/// [`Option`]: https://doc.rust-lang.org/std/option/enum.Option.html
/// [`OptionLogFormatter`]: crate::option_logger::OptionLogFormatter
pub trait OptionLogger<T> {
    /// Output a log message of the provided severity if `Some` or `None`. If `Some`, the wrapped
    /// value will be appended.
    fn log(self, level: Level, message: &str) -> Option<T>;

    /// Output a log message of the provided severity if `Some`, with the wrapped value appended.
    fn log_some(self, level: Level, message: &str) -> Option<T>;

    /// Output a log message of the provided severity if `None`.
    fn log_none(self, level: Level, message: &str) -> Option<T>;

    option_log_trait_group!(trace);
    option_log_trait_group!(debug);
    option_log_trait_group!(info);
    option_log_trait_group!(warn);
    option_log_trait_group!(error);
}

/// Augment [`Option`] types with log methods that can print abitrary representations of wrapped
/// values. If the type being wrapped implements `Debug`, you may be able to use [`OptionLogger`]
/// instead.
///
/// The callsite is responsible for defining the format strategy. The closure is only evaluated
/// when necessary.
///
/// This trait defines methods prefixed by the log level, as well as a general one that accepts a
/// log level parameter.
///
/// * `<log_level>_format(FnOnce)` prints the return value of the closure if `Some` or `None`
/// * `<log_level>_format_some(FnOnce)` prints the return value of the closure if `Some`
/// * `<log_level>_format_none(FnOnce)` prints the return value of the closure if `None`
///
/// [`Option`]: https://doc.rust-lang.org/std/option/enum.Option.html
/// [`OptionLogger`]: crate::option_logger::OptionLogger
pub trait OptionLogFormatter<T> {
    /// Output a log message of the provided severity if `Some` or `None`. The message is
    /// determined by the return value of the supplied closure.
    fn log_format<F>(self, level: Level, f: F) -> Option<T>
    where
        F: FnOnce(&Option<T>) -> String;

    /// Output a log message of the provided severity if `Some`. The message is determined by the
    /// return value of the supplied closure.
    fn log_some_format<F>(self, level: Level, f: F) -> Option<T>
    where
        F: FnOnce(&T) -> String;

    /// Output a log message of the provided severity if `None`. The message is determined by the
    /// return value of the supplied closure.
    fn log_none_format<F>(self, level: Level, f: F) -> Option<T>
    where
        F: FnOnce() -> String;

    option_format_trait_group!(trace);
    option_format_trait_group!(debug);
    option_format_trait_group!(info);
    option_format_trait_group!(warn);
    option_format_trait_group!(error);
}

macro_rules! option_log_impl_group {
    ($level_name:ident, $level:expr) => {
        paste! {
            fn $level_name(self, message: &str) -> Option<T> {
                self.log($level, message)
            }

            fn [<$level_name _some>](self, message: &str) -> Option<T> {
                self.log_some($level, message)
            }

            fn [<$level_name _none>](self, message: &str) -> Option<T> {
                self.log_none($level, message)
            }
        }
    };
}

macro_rules! option_format_impl_group {
    ($level_name:ident, $level:expr) => {
        paste! {
            fn [<$level_name _format>]<F>(self, f: F) -> Option<T>
            where
                F: FnOnce(&Option<T>) -> String,
            {
                self.log_format($level, f)
            }

            fn [<$level_name _some_format>]<F>(self, f: F) -> Option<T>
            where
                F: FnOnce(&T) -> String,
            {
                self.log_some_format($level, f)
            }

            fn [<$level_name _none_format>]<F>(self, f: F) -> Option<T>
            where
                F: FnOnce() -> String,
            {
                self.log_none_format($level, f)
            }
        }
    };
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

    fn log_some(self, level: Level, message: &str) -> Option<T> {
        self.map(|v| {
            log!(level, "{}: {:?}", message, v);
            v
        })
    }

    fn log_none(self, level: Level, message: &str) -> Option<T> {
        if self.is_none() {
            log!(level, "{}: None", message);
        }
        self
    }

    option_log_impl_group!(trace, Level::Trace);
    option_log_impl_group!(debug, Level::Debug);
    option_log_impl_group!(info, Level::Info);
    option_log_impl_group!(warn, Level::Warn);
    option_log_impl_group!(error, Level::Error);
}

impl<T> OptionLogFormatter<T> for Option<T> {
    fn log_format<F>(self, level: Level, f: F) -> Option<T>
    where
        F: FnOnce(&Option<T>) -> String,
    {
        if log::log_enabled!(level) {
            log!(level, "{}", f(&self));
        }
        self
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

    fn log_none_format<F>(self, level: Level, f: F) -> Option<T>
    where
        F: FnOnce() -> String,
    {
        if log::log_enabled!(level) && self.is_none() {
            log!(level, "{}", f());
        }
        self
    }

    option_format_impl_group!(trace, Level::Trace);
    option_format_impl_group!(debug, Level::Debug);
    option_format_impl_group!(info, Level::Info);
    option_format_impl_group!(warn, Level::Warn);
    option_format_impl_group!(error, Level::Error);
}
