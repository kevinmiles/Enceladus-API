# Rust

<!-- maintainer: @jhpratt -->

```rust,ignore
struct Event {
  id: i32,
  posted: bool,
  in_thread_id: i32,
  // The API guarantees the `cols` field is an array containing strings and/or numbers
  cols: serde_json::Value,
}

struct Section {
  id: i32,
  is_events_section: bool,
  name: String,
  content: String,
  lock_held_by_user_id: Option<i32>,
  in_thread_id: i32,
  lock_assigned_at_utc: i64,
}

struct Thread {
  id: i32,
  thread_name: String,
  display_name: String,
  post_id: Option<String>,
  subreddit: Option<String>,
  space__t0: Option<i64>,
  youtube_id: Option<String>,
  spacex__api_id: Option<String>,
  created_by_user_id: i32,
  sections_id: Vec<i32>,
  events_id: Vec<i32>,
  event_column_headers: Vec<String>,
  space__utc_col_index: Option<i16>,
}

struct User {
  id: i32,
  reddit_username: String,
  lang: String,
  refresh_token: String,
  is_global_admin: bool,
  spacex__is_admin: bool,
  spacex__is_mod: bool,
  spacex__is_slack_member: bool,
}
```
