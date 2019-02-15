use crate::Database;
use std::error::Error;

const EVENT_CACHE_SIZE: usize = 100;
const SECTION_CACHE_SIZE: usize = 50;
const THREAD_CACHE_SIZE: usize = 5;
const USER_CACHE_SIZE: usize = 100;

pub trait ToMarkdown {
    fn to_markdown(&self, conn: &Database) -> Result<String, Box<dyn Error>>;
}

mod claim;
mod event;
mod section;
mod thread;
mod user;

pub use claim::*;
pub use event::*;
pub use section::*;
pub use thread::*;
pub use user::*;
