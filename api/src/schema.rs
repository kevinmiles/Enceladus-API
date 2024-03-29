table! {
    event (id) {
        id -> Int4,
        posted -> Bool,
        in_thread_id -> Int4,
        cols -> Jsonb,
    }
}

table! {
    section (id) {
        id -> Int4,
        is_events_section -> Bool,
        name -> Varchar,
        content -> Text,
        lock_held_by_user_id -> Nullable<Int4>,
        in_thread_id -> Int4,
        lock_assigned_at_utc -> Int8,
    }
}

table! {
    thread (id) {
        id -> Int4,
        thread_name -> Varchar,
        display_name -> Varchar,
        post_id -> Nullable<Varchar>,
        subreddit -> Nullable<Varchar>,
        space__t0 -> Nullable<Int8>,
        video_url -> Nullable<Varchar>,
        spacex__api_id -> Nullable<Varchar>,
        created_by_user_id -> Int4,
        sections_id -> Array<Int4>,
        events_id -> Array<Int4>,
        event_column_headers -> Array<Text>,
        space__utc_col_index -> Nullable<Int2>,
        is_live -> Bool,
    }
}

table! {
    user (id) {
        id -> Int4,
        reddit_username -> Text,
        lang -> Varchar,
        refresh_token -> Bytea,
        is_global_admin -> Bool,
        spacex__is_host -> Bool,
        spacex__is_mod -> Bool,
        spacex__is_slack_member -> Bool,
        access_token -> Bytea,
        access_token_expires_at_utc -> Int8,
    }
}

joinable!(section -> user (lock_held_by_user_id));
joinable!(thread -> user (created_by_user_id));

allow_tables_to_appear_in_same_query!(
    event,
    section,
    thread,
    user,
);
