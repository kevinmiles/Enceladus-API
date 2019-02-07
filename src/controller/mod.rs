const EVENT_CACHE_SIZE: usize = 100;
const PRESET_EVENT_CACHE_SIZE: usize = 100;
const SECTION_CACHE_SIZE: usize = 50;
const THREAD_CACHE_SIZE: usize = 5;
const USER_CACHE_SIZE: usize = 100;

pub mod claim;
pub mod event;
pub mod global_admin;
pub mod preset_event;
pub mod section;
pub mod thread;
pub mod user;
