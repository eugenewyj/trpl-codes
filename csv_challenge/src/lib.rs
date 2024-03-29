//! This is documentation for the `csv_challenge` lib crate
//! 
//! 用例：
//! ```
//!     use csv_challenge::{
//!         Opt,
//!         {load_csv, write_csv},
//!         replace_column,
//!     };
//! ```
mod opt;
mod err;
mod core;

pub use self::opt::Opt;
pub use self::core::{
    read::{load_csv, write_csv},
    write::replace_column,
};