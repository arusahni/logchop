#![warn(missing_docs)]
#![crate_name="logchop"]

//! Log your `Result` and `Option` chains with inline methods.
//!
//! First, import all traits:
//!
//! ```
//! use logchop::*;
//! ```
//!
//! This will extend `Option` and `Result` with new methods. Now
//! you can turn:
//!
//! ```rust
//! return match maybe_something {
//!     Some(x) => Some(x),
//!     None => {
//!         debug!("Nothing found!");
//!         None
//!     }
//! }
//! ```
//!
//! into
//!
//! ```rust
//! return maybe_something.debug_none("Nothing found!")
//! ```
//!
//! 😗🤌
//!
//! The full list of methods exposed are available on the respective types
//!
//! * [`OptionLogger`](crate::option_logger::OptionLogger)
//! * [`OptionLogFormatter`](crate::option_logger::OptionLogFormatter)
//! * [`ResultLogger`](crate::result_logger::ResultLogger)
//! * [`ResultLogFormatter`](crate::result_logger::ResultLogFormatter)

mod option_logger;
mod result_logger;

pub use option_logger::*;
pub use result_logger::*;
