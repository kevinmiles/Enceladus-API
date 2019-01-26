macro_rules! find_inserted {
    ($table:ident, $conn:ident) => {
        $table
            .filter(
                $table::id.eq::<i32>(
                    $table
                        .select($table::id)
                        .order($table::id.desc())
                        .first($conn)
                        .unwrap(),
                ),
            )
            .first($conn)
            .expect("Could not find inserted row")
    };
}

pub mod event;
pub mod preset_event;
pub mod section;
pub mod thread;
pub mod user;
