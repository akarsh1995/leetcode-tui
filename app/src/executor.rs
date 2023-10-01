use config::key::Key;

use crate::ctx::Ctx;

pub struct Executor;

impl Executor {
    pub fn handle(cx: &mut Ctx, key: Key) -> bool {
        if cx.popup.visible {
            return match key {
                Key::Enter => cx.popup.toggle(),
                _ => false,
            };
        }

        if cx.topic.visible {
            return match key {
                Key::Char('T') => cx.topic.prev_topic(),
                Key::Char('t') => cx.topic.next_topic(),
                Key::Up => cx.question.prev_ques(),
                Key::Down => cx.question.next_ques(),
                Key::Enter => cx.question.show_question_content(),
                _ => false,
            };
        }
        false
    }
}
