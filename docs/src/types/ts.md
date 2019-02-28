# TypeScript

<!-- maintainer: @jhpratt -->

```typescript
type Event = {
  id: number;
  posted: boolean;
  in_thread_id: number;
  cols: (string | number)[];
};

type Section = {
  id: number;
  is_events_section: boolean;
  name: string;
  content: string;
  lock_held_by_user_id: i32 | null;
  in_thread_id: number;
  lock_assigned_at_utc: number;
};

type Thread = {
  id: number;
  thread_name: string;
  display_name: string;
  post_id: string | null;
  subreddit: string | null;
  space__t0: number | null;
  youtube_id: string | null;
  spacex__api_id: string | null;
  created_by_user_id: number;
  sections_id: number[];
  events_id: number[];
  event_column_headers: string[];
  space__utc_col_index: number | null;
};

type User = {
  id: number;
  reddit_username: string;
  lang: string;
  is_global_admin: boolean;
  spacex__is_admin: boolean;
  spacex__is_mod: boolean;
  spacex__is_slack_member: boolean;
};
```
