use hashbrown::HashMap;
use lazy_static::lazy_static;
use parking_lot::RwLock;
use serde_json::{json, Value as Json};
use std::sync::{Arc, Weak};
use ws::{CloseCode, Handler, Handshake, Message, Result, Sender};

mod structs;
use structs::*;

lazy_static! {
    // FIXME Change this `Vec` to a `HashSet` or `BTreeSet`
    // as soon as upstream changes allow.
    // The current implementation makes removing an entry quite expensive.
    static ref ROOMS: RwLock<HashMap<Room, Vec<Weak<Sender>>>> = RwLock::new(HashMap::new());
}

#[cfg(debug_assertions)]
const IP: &str = "127.0.0.1";
#[cfg(not(debug_assertions))]
const IP: &str = "0.0.0.0";
const PORT: u16 = 3001;

struct Socket {
    out:   Arc<Sender>,
    rooms: HashMap<Room, usize>,
}

impl Handler for Socket {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        println!("client has connected");
        Ok(())
    }

    fn on_message(&mut self, message: Message) -> Result<()> {
        let message = match message {
            Message::Text(s) => s,
            _ => return Ok(()),
        };

        let mut rooms = ROOMS.write();

        for room in match serde_json::from_str(&message) {
            Ok(JoinRequest { join }) => join,
            _ => return Ok(()),
        }
        .into_iter()
        .filter_map(Room::from_string)
        {
            let room_set = match rooms.get_mut(&room) {
                Some(s) => s,
                None => {
                    rooms.insert(room.clone(), vec![]);
                    rooms.get_mut(&room).unwrap()
                }
            };
            room_set.push(Arc::downgrade(&self.out));
            self.rooms.insert(room, room_set.len() - 1);
        }

        self.out.close(CloseCode::Normal)
    }

    fn on_close(&mut self, _code: CloseCode, _reason: &str) {
        // Avoid locking the map if we don't need to.
        if !self.rooms.is_empty() {
            let mut rooms = ROOMS.write();

            for (room, &index) in self.rooms.iter() {
                rooms.get_mut(room).unwrap().remove(index);
            }
        }

        println!("client has disconnected");
    }
}

#[allow(dead_code)]
pub fn send_message_to_room(
    room: &Room,
    action: &Action,
    data_type: &DataType,
    message: &Json,
) -> Result<()> {
    let rooms = ROOMS.read();
    let clients = match rooms.get(room) {
        Some(v) => v,
        None => return Ok(()),
    };

    let message = &*json!({
        "room": room,
        "action": action,
        "data_type": data_type,
        "data": message,
    })
    .to_string();

    let _ = clients.iter().filter_map(Weak::upgrade).inspect(|client| {
        let _ = client.send(message);
    });

    Ok(())
}

pub fn spawn() {
    let addr = format!("{}:{}", IP, PORT);
    ws::listen(addr, |out| Socket {
        out:   Arc::new(out),
        rooms: HashMap::new(),
    })
    .unwrap();
}
