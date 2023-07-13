CREATE TABLE IF NOT EXISTS TopicTag (
    name TEXT,
    id TEXT,
    slug TEXT,
    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS Question (
    ac_rate REAL,
    difficulty TEXT,
    freq_bar REAL,
    frontend_question_id TEXT,
    is_favor INTEGER,
    paid_only INTEGER,
    status TEXT,
    title TEXT,
    title_slug TEXT,
    has_solution INTEGER,
    has_video_solution INTEGER,
    PRIMARY KEY (frontend_question_id)
);

CREATE TABLE IF NOT EXISTS QuestionTopicTag (
    question_id TEXT,
    tag_id TEXT,
    FOREIGN KEY (question_id) REFERENCES Question (frontend_question_id),
    FOREIGN KEY (tag_id) REFERENCES TopicTag (id),
    PRIMARY KEY (question_id, tag_id)
);
