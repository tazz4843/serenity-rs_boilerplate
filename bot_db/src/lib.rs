#![feature(once_cell)]
#![feature(option_result_unwrap_unchecked)]

mod dyn_prefix;
mod load_database;
mod type_map_key;

pub use dyn_prefix::*;
pub use load_database::*;
pub use type_map_key::*;

use sqlx::{Pool, Postgres};
use std::lazy::SyncOnceCell as OnceCell;

pub const DATABASE_ENABLED: bool = true;
pub static PG_POOL: OnceCell<Pool<Postgres>> = OnceCell::new();
