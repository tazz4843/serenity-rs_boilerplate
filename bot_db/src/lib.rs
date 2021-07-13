#![feature(once_cell)]
#![feature(option_result_unwrap_unchecked)]

mod load_database;
mod type_map_key;
mod dyn_prefix;

pub use load_database::*;
pub use type_map_key::*;
pub use dyn_prefix::*;

use sqlx::{Pool, Postgres};
use std::lazy::SyncOnceCell as OnceCell;

pub const DATABASE_ENABLED: bool = true;
pub static PG_POOL: OnceCell<Pool<Postgres>> = OnceCell::new();
