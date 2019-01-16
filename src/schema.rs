table! {
    preset_event (id) {
        id -> Integer,
        holds_clock -> Bool,
        message -> Text,
        name -> Text,
    }
}

table! {
    user (id) {
        id -> Integer,
        reddit_username -> Text,
        lang -> Text,
        refresh_token -> Text,
        is_global_admin -> Bool,
        spacex__is_admin -> Bool,
        spacex__is_mod -> Bool,
        spacex__is_slack_member -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(preset_event, user);
