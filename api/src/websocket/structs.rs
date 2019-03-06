use serde::{Deserialize, Serialize, Serializer};

#[derive(Deserialize, Debug)]
pub struct JoinRequest {
    pub join: Vec<String>,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum Room {
    User,
    ThreadCreate,
    Thread(i32),
}

impl Serialize for Room {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use Room::*;
        serializer.serialize_str(&match self {
            User => "user".to_owned(),
            ThreadCreate => "thread_create".to_owned(),
            Thread(id) => format!("thread_{}", id),
        })
    }
}

impl Room {
    #[inline]
    pub fn from_string(string: String) -> Option<Room> {
        match &*string {
            "user" => Room::User.into(),
            "thread_create" => Room::ThreadCreate.into(),
            room if room.starts_with("thread_") => match room[7..].parse() {
                Ok(id) => Room::Thread(id).into(),
                Err(_) => None,
            },
            _ => None,
        }
    }
}

pub enum Action {
    Create,
    Update,
    Delete,
}

impl Serialize for Action {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use Action::*;
        serializer.serialize_str(match self {
            Create => "create",
            Update => "update",
            Delete => "delete",
        })
    }
}

pub enum DataType {
    Event,
    Section,
    Thread,
    User,
}

impl Serialize for DataType {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use DataType::*;
        serializer.serialize_str(match self {
            Event => "event",
            Section => "section",
            Thread => "thread",
            User => "user",
        })
    }
}
