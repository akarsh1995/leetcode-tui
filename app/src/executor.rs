use config::key::Key;

use crate::ctx::Ctx;

pub struct Executor;

impl Executor {
    pub fn handle(cx: &mut Ctx, key: Key) -> bool {
        match key {
            Key::Up | Key::Char('k') => cx.topic.prev(),
            Key::Down | Key::Char('j') => cx.topic.next(),
            _ => false,
        }
    }
}
