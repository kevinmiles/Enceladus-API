table! {
    preset_event (id) {
        id -> Int4,
        holds_clock -> Bool,
        message -> Text,
        name -> Varchar,
    }
}

table! {
    thread (id) {
        id -> Int4,
        thread_name -> Varchar,
        launch_name -> Varchar,
        post_id -> Nullable<Varchar>,
        subreddit -> Varchar,
        t0 -> Nullable<Int8>,
        youtube_id -> Nullable<Varchar>,
        spacex__api_id -> Nullable<Varchar>,
        created_by_user_id -> Int4,
        sections_id -> Array<Int4>,
        events_id -> Array<Int4>,
    }
}

table! {
    user (id) {
        id -> Int4,
        reddit_username -> Text,
        lang -> Varchar,
        refresh_token -> Text,
        is_global_admin -> Bool,
        spacex__is_admin -> Bool,
        spacex__is_mod -> Bool,
        spacex__is_slack_member -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    preset_event,
    thread,
    user,
);
