use std::{cell::RefCell, collections::HashMap, fmt::Display, rc::Rc};

use indexmap::IndexMap;

use crate::{
    app_ui::async_task_channel::Response,
    deserializers::editor_data,
    deserializers::{
        question_content::QuestionContent,
        run_submit::{self, ParsedResponse, Success},
    },
    entities::{QuestionModel, TopicTagModel},
};

use super::{CachedQuestion, Question};

pub(super) fn process_get_all_question_map_task_content(
    content: HashMap<TopicTagModel, Vec<QuestionModel>>,
    topic_tag_question_map: &mut HashMap<Rc<TopicTagModel>, Vec<super::Question>>,
    question_id_question_map: &mut IndexMap<String, super::Question>,
) {
    // creating rc cloned question as one question can appear in multiple topics
    // create (frontend_question_id, QuestionModel) mapping
    let question_set = content
        .iter()
        .flat_map(|x| {
            x.1.iter().map(|x| {
                (
                    x.frontend_question_id.clone(),
                    Rc::new(RefCell::new(x.clone())),
                )
            })
        })
        .collect::<IndexMap<_, _>>();

    // (topic_tag, question_mapping)
    let map_iter = content.into_iter().map(|v| {
        (
            Rc::new(v.0),
            (v.1.into_iter()
                .map(|x| question_set[&x.frontend_question_id].clone()))
            .collect::<Vec<_>>(),
        )
    });

    let all_questions = question_set.values().cloned().collect();

    topic_tag_question_map.extend(map_iter);
    topic_tag_question_map.extend(vec![(
        Rc::new(TopicTagModel {
            name: "All".to_owned(),
            id: "all".to_owned(),
            slug: "all".to_owned(),
        }),
        all_questions,
    )]);

    for ql in topic_tag_question_map.values_mut() {
        ql.sort_unstable()
    }

    *question_id_question_map = question_set;
    question_id_question_map.sort_by(|_, y, _, k| y.cmp(k));
}

pub(super) fn process_question_detail_response<'a>(
    response: Response<QuestionContent>,
    question: &Question,
    cache: &'a mut lru::LruCache<String, super::CachedQuestion>,
) -> &'a CachedQuestion {
    let cached_q = cache.get_or_insert_mut(
        question.borrow().frontend_question_id.clone(),
        super::CachedQuestion::default,
    );
    cached_q.qd = Some(response.content);
    cached_q
}

pub(super) fn process_question_editor_data<'a>(
    response: Response<editor_data::Question>,
    question: &Question,
    cache: &'a mut lru::LruCache<String, super::CachedQuestion>,
) -> &'a CachedQuestion {
    let cached_q = cache.get_or_insert_mut(
        question.borrow().frontend_question_id.clone(),
        super::CachedQuestion::default,
    );
    cached_q.editor_data = Some(response.content);
    cached_q
}

impl Display for run_submit::ParsedResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match &self {
            ParsedResponse::Pending => "Pending".to_string(),
            ParsedResponse::CompileError(_) => "Compile Error".to_string(),
            ParsedResponse::RuntimeError(re) => re.full_runtime_error.to_string(),
            ParsedResponse::MemoryLimitExceeded(_) => "Memory Limit Exceeded".to_string(),
            ParsedResponse::OutputLimitExceed(_) => "Output Limit Exceeded".to_string(),
            ParsedResponse::TimeLimitExceeded(_) => "Time Limit Exceeded".to_string(),
            ParsedResponse::InternalError(_) => "Internal Error".to_string(),
            ParsedResponse::TimeOut(_) => "Timout".to_string(),
            ParsedResponse::Unknown(_) => "Unknown".to_string(),
            ParsedResponse::Success(Success::Run {
                status_runtime,
                code_answer,
                expected_code_answer,
                correct_answer,
                total_correct,
                total_testcases,
                status_memory,
                ..
            }) => {
                let is_accepted_symbol = if *correct_answer { "✅" } else { "❌" };
                let mut ans_compare = String::new();
                for (output, expected_output) in code_answer.iter().zip(expected_code_answer) {
                    let emoji = if output == expected_output {
                        "✅"
                    } else {
                        "❌"
                    };
                    let compare = format!(
                        "{emoji}\nOuput: {}\nExpected: {}\n\n",
                        output, expected_output
                    );
                    ans_compare.push_str(compare.as_str())
                }
                let result_string = vec![
                    format!("Accepted: {}", is_accepted_symbol),
                    if let Some(correct) = total_correct {
                        let mut x = format!("Correct: {correct}");
                        if let Some(total) = total_testcases {
                            x = format!("{x}/{}", total);
                        }
                        x
                    } else {
                        String::new()
                    },
                    format!("Memory Used: {status_memory}"),
                    format!("Status Runtime: {status_runtime}"),
                    ans_compare,
                ];
                result_string.join("\n")
            }
            ParsedResponse::Success(Success::Submit {
                status_runtime,
                total_correct,
                total_testcases,
                status_memory,
                ..
            }) => {
                let is_accepted_symbol = "✅";
                let result_string = vec![
                    format!("Accepted: {}", is_accepted_symbol),
                    if let Some(correct) = total_correct {
                        let mut x = format!("Correct: {correct}");
                        if let Some(total) = total_testcases {
                            x = format!("{x}/{}", total);
                        }
                        x
                    } else {
                        String::new()
                    },
                    format!("Memory Used: {status_memory}"),
                    format!("Status Runtime: {status_runtime}"),
                ];
                result_string.join("\n")
            }
        };
        f.write_fmt(format_args!("{string}"))
    }
}
