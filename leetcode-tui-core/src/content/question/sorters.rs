use std::rc::Rc;

use leetcode_tui_db::DbQuestion;

pub(crate) fn sort_by_likes_dislikes_ratio(questions_haystack: &mut Vec<Rc<DbQuestion>>) {
    questions_haystack.sort_by_key(|q| {
        if q.dislikes == 0 {
            return q.likes;
        } else {
            q.likes / q.dislikes
        }
    });
}

pub(crate) fn sort_by_id(questions_haystack: &mut Vec<Rc<DbQuestion>>) {
    questions_haystack.sort_by_key(|q| q.id);
}

pub(crate) fn sort_by_difficulty(questions_haystack: &mut Vec<Rc<DbQuestion>>) {
    questions_haystack.sort_by_key(|q| q.difficulty.clone());
}

pub(crate) fn sort_by_acceptance_rate(questions_haystack: &mut Vec<Rc<DbQuestion>>) {
    questions_haystack.sort_by_key(|q| {
        if let Some(ac) = q.acceptance_rate {
            ac as i32
        } else {
            return 0;
        }
    });
}
