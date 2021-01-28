use log::{log, Level};
use paste::paste;

macro_rules! option_trait_group {
    ($level:ident) => {
        paste! {
            fn $level(self, message: &str) -> Option<T>;

            fn [<$level _format>]<F>(self, f: F) -> Option<T>
            where
                F: FnOnce(&Option<T>) -> String;

            fn [<$level _some>](self, message: &str) -> Option<T>;

            fn [<$level _some_format>]<F>(self, f: F) -> Option<T>
            where
                F: FnOnce(&T) -> String;

            fn [<$level _none>](self, message: &str) -> Option<T>;

            fn [<$level _none_format>]<F>(self, f: F) -> Option<T>
            where
                F: FnOnce() -> String;
        }
    };
}

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

    option_trait_group!(trace);
    option_trait_group!(debug);
    option_trait_group!(info);
    option_trait_group!(warn);
    option_trait_group!(error);
}

macro_rules! result_trait_group {
    ($level:ident) => {
        paste! {
            fn $level(self, message: &str) -> Result<T, E>;

            fn [<$level _format>]<F>(self, f: F) -> Result<T, E>
            where
                F: FnOnce(&Result<T, E>) -> String;

            fn [<$level _ok>](self, message: &str) -> Result<T, E>;

            fn [<$level _ok_format>]<F>(self, f: F) -> Result<T, E>
            where
                F: FnOnce(&T) -> String;

            fn [<$level _err>](self, message: &str) -> Result<T, E>;

            fn [<$level _err_format>]<F>(self, f: F) -> Result<T, E>
            where
                F: FnOnce(&E) -> String;
        }
    };
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

    result_trait_group!(trace);
    result_trait_group!(debug);
    result_trait_group!(info);
    result_trait_group!(warn);
    result_trait_group!(error);
}

macro_rules! option_impl_group {
    ($level_name:ident, $level:expr) => {
        paste! {
            fn $level_name(self, message: &str) -> Option<T> {
                self.log($level, message)
            }

            fn [<$level_name _format>]<F>(self, f: F) -> Option<T>
            where
                F: FnOnce(&Option<T>) -> String,
            {
                self.log_format($level, f)
            }

            fn [<$level_name _some>](self, message: &str) -> Option<T> {
                self.log_some($level, message)
            }

            fn [<$level_name _some_format>]<F>(self, f: F) -> Option<T>
            where
                F: FnOnce(&T) -> String,
            {
                self.log_some_format($level, f)
            }

            fn [<$level_name _none>](self, message: &str) -> Option<T> {
                self.log_none($level, message)
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

    option_impl_group!(trace, Level::Trace);
    option_impl_group!(debug, Level::Debug);
    option_impl_group!(info, Level::Info);
    option_impl_group!(warn, Level::Warn);
    option_impl_group!(error, Level::Error);
}

macro_rules! result_impl_group {
    ($level_name:ident, $level:expr) => {
        paste! {
            fn $level_name(self, message: &str) -> Result<T, E> {
                self.log($level, message)
            }

            fn [<$level_name _format>]<F>(self, f: F) -> Result<T, E>
            where
                F: FnOnce(&Result<T, E>) -> String,
            {
                self.log_format($level, f)
            }

            fn [<$level_name _ok>](self, message: &str) -> Result<T, E> {
                self.log_ok($level, message)
            }

            fn [<$level_name _ok_format>]<F>(self, f: F) -> Result<T, E>
            where
                F: FnOnce(&T) -> String,
            {
                self.log_ok_format($level, f)
            }

            fn [<$level_name _err>](self, message: &str) -> Result<T, E> {
                self.log_err($level, message)
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

    result_impl_group!(trace, Level::Trace);
    result_impl_group!(debug, Level::Debug);
    result_impl_group!(info, Level::Info);
    result_impl_group!(warn, Level::Warn);
    result_impl_group!(error, Level::Error);
}
