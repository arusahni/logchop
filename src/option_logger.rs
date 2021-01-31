use log::{log, Level};
use paste::paste;

macro_rules! option_log_trait_group {
    ($level:ident) => {
        paste! {
            fn $level(self, message: &str) -> Option<T>;

            fn [<$level _some>](self, message: &str) -> Option<T>;

            fn [<$level _none>](self, message: &str) -> Option<T>;
        }
    };
}

macro_rules! option_format_trait_group {
    ($level:ident) => {
        paste! {
            fn [<$level _format>]<F>(self, f: F) -> Option<T>
            where
                F: FnOnce(&Option<T>) -> String;

            fn [<$level _some_format>]<F>(self, f: F) -> Option<T>
            where
                F: FnOnce(&T) -> String;

            fn [<$level _none_format>]<F>(self, f: F) -> Option<T>
            where
                F: FnOnce() -> String;
        }
    };
}

pub trait OptionLogger<T> {
    fn log(self, level: Level, message: &str) -> Option<T>;
    fn log_some(self, level: Level, message: &str) -> Option<T>;
    fn log_none(self, level: Level, message: &str) -> Option<T>;

    option_log_trait_group!(trace);
    option_log_trait_group!(debug);
    option_log_trait_group!(info);
    option_log_trait_group!(warn);
    option_log_trait_group!(error);
}

pub trait OptionLogFormatter<T> {
    fn log_format<F>(self, level: Level, f: F) -> Option<T>
    where
        F: FnOnce(&Option<T>) -> String;

    fn log_some_format<F>(self, level: Level, f: F) -> Option<T>
    where
        F: FnOnce(&T) -> String;

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
