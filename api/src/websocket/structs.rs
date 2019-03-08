use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{fmt, sync::Weak};

/// A request from a client to join certain rooms.
/// Each element should be able to be parsed with `Room::from_str`.
#[derive(Deserialize, Debug)]
pub struct JoinRequest {
    pub join: Vec<String>,
}

/// Room to send a `Message` to.
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum Room {
    User,
    ThreadCreate,
    Thread(i32),
}

impl fmt::Display for Room {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Room::*;
        write!(
            f,
            "{}",
            match self {
                User => "user".to_owned(),
                ThreadCreate => "thread_create".to_owned(),
                Thread(id) => format!("thread_{}", id),
            }
        )
    }
}

impl std::str::FromStr for Room {
    type Err = &'static str;

    #[inline]
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string {
            "user" => Ok(Room::User),
            "thread_create" => Ok(Room::ThreadCreate),
            room if room.starts_with("thread_") => match room[7..].parse() {
                Ok(id) => Ok(Room::Thread(id)),
                Err(_) => Err("invalid thread id"),
            },
            _ => Err("unknown room name"),
        }
    }
}

/// What action is the `data` field representing in a `Message`?
pub enum Action {
    Create,
    Update,
    Delete,
}

impl fmt::Display for Action {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Action::*;
        write!(
            f,
            "{}",
            match self {
                Create => "create",
                Update => "update",
                Delete => "delete",
            }
        )
    }
}

/// What type is the `data` field in a `Message`?
pub enum DataType {
    Event,
    Section,
    Thread,
    User,
}

impl fmt::Display for DataType {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use DataType::*;
        write!(
            f,
            "{}",
            match self {
                Event => "event",
                Section => "section",
                Thread => "thread",
                User => "user",
            }
        )
    }
}

/// A message that can be emitted to the various WebSocket clients.
/// Any serializable type can be sent as `data`,
/// though it should match the indicated `data_type`.
pub struct Message<'a, T: Serialize> {
    pub room:      Room,
    pub action:    Action,
    pub data_type: DataType,
    pub data:      &'a T,
}

impl<T: Serialize> Message<'_, T> {
    #[inline]
    pub fn send(&self) -> ws::Result<()> {
        let rooms = super::ROOMS.read();
        let clients = match rooms.get(&self.room) {
            Some(v) => v,
            None => return Ok(()),
        };

        let message = &*json!({
            "room": self.room.to_string(),
            "action": self.action.to_string(),
            "data_type": self.data_type.to_string(),
            "data": self.data,
        })
        .to_string();

        for client in clients.iter().filter_map(Weak::upgrade) {
            let _ = client.send(message);
        }

        Ok(())
    }
}

/// Use this struct to add an `id` field to a preexisting struct.
/// The fields will be flattened by serde.
#[derive(Serialize)]
pub struct Update<'a, T: Serialize> {
    pub id: i32,
    #[serde(flatten)]
    pub data: &'a T,
}

impl<'a, T: Serialize> Update<'a, T> {
    #[inline(always)]
    pub fn new(id: i32, data: &'a T) -> Self {
        Self { id, data }
    }
}