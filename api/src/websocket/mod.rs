use hashbrown::HashMap;
use lazy_static::lazy_static;
use parking_lot::RwLock;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
    Weak,
};
use ws::{CloseCode, Handler, Handshake, Message as WsMessage, Result, Sender};

mod structs;
pub use structs::*;

lazy_static! {
    // FIXME Change this `Vec` to a `HashSet` or `BTreeSet`
    // as soon as upstream changes allow.
    // The current implementation makes removing an entry quite expensive.
    static ref ROOMS: RwLock<HashMap<Room, Vec<Weak<Sender>>>> = RwLock::new(HashMap::new());
}

pub static CONNECTED_CLIENTS: AtomicUsize = AtomicUsize::new(0);

#[cfg(debug_assertions)]
const IP: &str = "127.0.0.1";
#[cfg(not(debug_assertions))]
const IP: &str = "0.0.0.0";
const PORT: u16 = 3001;

#[derive(Debug)]
struct Socket {
    out:   Arc<Sender>,
    rooms: HashMap<Room, usize>,
}

impl Handler for Socket {
    #[inline(always)]
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        CONNECTED_CLIENTS.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }

    #[inline]
    fn on_message(&mut self, message: WsMessage) -> Result<()> {
        let message = match message {
            WsMessage::Text(s) => s,
            _ => return Ok(()),
        };

        let mut rooms = ROOMS.write();

        for room in match serde_json::from_str(&message) {
            Ok(JoinRequest { join }) => join,
            _ => return Ok(()),
        }
        .into_iter()
        .filter_map(|s| s.parse().ok())
        {
            let room_set = match rooms.get_mut(&room) {
                Some(s) => s,
                None => {
                    rooms.insert(room, vec![]);
                    rooms.get_mut(&room).unwrap()
                }
            };
            room_set.push(Arc::downgrade(&self.out));
            self.rooms.insert(room, room_set.len() - 1);
        }

        Ok(())
    }

    #[inline]
    fn on_close(&mut self, _code: CloseCode, _reason: &str) {
        // Avoid locking the map if we don't need to.
        if !self.rooms.is_empty() {
            let mut rooms = ROOMS.write();

            for (room, &index) in self.rooms.iter() {
                rooms.get_mut(room).unwrap().remove(index);
            }
        }

        CONNECTED_CLIENTS.fetch_sub(1, Ordering::Relaxed);
    }
}

#[inline]
pub fn spawn() {
    let addr = format!("{}:{}", IP, PORT);
    ws::listen(addr, |out| Socket {
        out:   Arc::new(out),
        rooms: HashMap::new(),
    })
    .unwrap();
}
