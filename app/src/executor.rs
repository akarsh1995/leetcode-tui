use config::key::Key;

use crate::ctx::Ctx;

pub struct Executor;

impl Executor {
    pub fn handle(cx: &mut Ctx, key: Key) -> bool {
        if cx.popup.visible {
            return match key {
                Key::Enter => cx.popup.toggle(),
                Key::Up | Key::Char('k') => cx.popup.scroll_up(),
                Key::Down | Key::Char('j') => cx.popup.scroll_down(),
                _ => false,
            };
        }

        if cx.select_popup.visible {
            return match key {
                Key::Enter => cx.select_popup.close(),
                Key::Esc => cx.select_popup.close_unselected(),
                Key::Up | Key::Char('k') => cx.select_popup.prev_item(),
                Key::Down | Key::Char('j') => cx.select_popup.next_item(),
                _ => false,
            };
        }

        if cx.input.visible {
            return match key {
                Key::Esc => cx.input.close(),
                Key::Char(c) => cx.input.char(c),
                Key::Backspace => cx.input.remove_char(),
                Key::Up | Key::Down => {
                    cx.input.close();
                    app_core::emit!(Key(key.into()));
                    true
                }
                _ => false,
            };
        }

        if cx.content.is_visible() {
            return match key {
                Key::Char('T') => cx.content.get_topic_mut().prev_topic(),
                Key::Char('t') => cx.content.get_topic_mut().next_topic(),
                Key::Char('e') => cx.content.get_questions_mut().solve_for_language(),
                Key::Up | Key::Char('k') => cx.content.get_questions_mut().prev_ques(),
                Key::Down | Key::Char('j') => cx.content.get_questions_mut().next_ques(),
                Key::Enter => cx.content.get_questions_mut().show_question_content(),
                Key::Char('r') => cx.content.get_questions_mut().run_solution(),
                Key::Char('s') => cx.content.get_questions_mut().submit_solution(),
                Key::Ctrl('s') => cx.content.get_questions_mut().toggle_stats(),
                Key::Char('/') => cx.content.get_questions_mut().toggle_search(),
                _ => false,
            };
        }
        false
    }
}
