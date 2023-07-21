use super::{notification::NotificationRequestSender, *};
use crate::app_ui::{channel::ChannelRequestSender, helpers::question};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Gauge},
};

#[derive(Debug)]
pub struct Stats {
    pub id: i32,
    pub task_sender: ChannelRequestSender,
    pub notification_sender: NotificationRequestSender,
    stat_state: Option<StatState>,
    pub active: bool,
}

impl Stats {
    pub(crate) fn new(
        id: i32,
        task_sender: ChannelRequestSender,
        notification_sender: NotificationRequestSender,
    ) -> Self {
        Self {
            id,
            task_sender,
            notification_sender,
            active: false,
            stat_state: None,
        }
    }
}

impl StateManager for Stats {
    fn set_active(&mut self) {
        self.active = true;
    }

    fn set_inactive(&mut self) {
        self.active = false;
    }

    fn is_active(&self) -> bool {
        self.active
    }
}

impl Stats {
    fn create_block(title: &str) -> Block {
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Gray))
            .title(Span::styled(
                title,
                Style::default().add_modifier(Modifier::BOLD),
            ))
    }
}

impl Widget for Stats {
    fn render(&mut self, rect: Rect, frame: &mut Frame<CrosstermBackend<Stderr>>) {
        let block = Self::create_block("Stats");
        let inner_area = block.inner(rect);
        frame.render_widget(block, rect);

        let horizontal_partition = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(inner_area);

        let left_partition = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(horizontal_partition[0]);

        let right_partition = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ])
            .split(horizontal_partition[1]);
        if let Some(stat_state) = &self.stat_state {
            let gauges: Vec<Gauge> = stat_state.into();
            for (part, gauge) in [
                left_partition[0],
                left_partition[1],
                right_partition[0],
                right_partition[1],
                right_partition[2],
            ]
            .into_iter()
            .zip(gauges)
            {
                frame.render_widget(gauge, part)
            }
        }
    }

    fn handler(&mut self, _event: KeyEvent) -> AppResult<()> {
        Ok(())
    }

    fn process_task_response(&mut self, _response: TaskResponse) -> AppResult<()> {
        Ok(())
    }

    fn set_response(&mut self) {}

    fn process_notification(&mut self, notification: &Notification) -> AppResult<()> {
        if let Notification::Stats(questions) = notification {
            let stats = crate::app_ui::helpers::question::Stats { qm: questions };
            self.stat_state = Some(stats.into());
        }
        Ok(())
    }
}

impl<'a> From<question::Stats<'a>> for StatState {
    fn from(val: question::Stats<'a>) -> Self {
        StatState {
            accepted: val.get_accepted(),
            total: val.get_total_question(),
            not_acepted: val.get_not_accepted(),
            not_attempted: val.get_not_attempted(),
            easy: val.get_easy_count(),
            medium: val.get_medium_count(),
            hard: val.get_hard_count(),
            easy_accepted: val.get_easy_accepted(),
            medium_accepted: val.get_medium_accepted(),
            hard_accepted: val.get_hard_accepted(),
        }
    }
}

#[derive(Debug)]
struct StatState {
    pub accepted: usize,
    pub total: usize,
    pub not_acepted: usize,
    pub not_attempted: usize,
    pub easy: usize,
    pub medium: usize,
    pub hard: usize,
    pub easy_accepted: usize,
    pub medium_accepted: usize,
    pub hard_accepted: usize,
}

impl StatState {
    fn get_gauge(title: &str, val: usize, total: usize, comination: Callout) -> Gauge {
        let block_title = format!("{}: {}/{}", title, val, total);
        let percentage = if total != 0 {
            (val as f32 / total as f32) * 100_f32
        } else {
            0 as f32
        };
        let style: Style = comination.get_pair().fg.into();
        let label = Span::styled(
            format!("{:.2}%", percentage),
            style.add_modifier(Modifier::ITALIC | Modifier::BOLD),
        );

        Gauge::default()
            .block(Block::default().title(block_title).borders(Borders::ALL))
            .gauge_style(Style::default().fg(Color::Green).bg(Color::Black))
            .percent(percentage as u16)
            .label(label)
    }
}

impl<'a> From<&StatState> for Vec<Gauge<'a>> {
    fn from(value: &StatState) -> Self {
        [
            ("Total Accepted", value.accepted, value.total, Callout::Info),
            (
                "Total Attempted",
                value.total - value.not_attempted,
                value.total,
                Callout::Info,
            ),
            (
                "Easy Accepted",
                value.easy_accepted,
                value.easy,
                Callout::Success,
            ),
            (
                "Medium Accepted",
                value.medium_accepted,
                value.medium,
                Callout::Warning,
            ),
            (
                "Hard Accepted",
                value.hard_accepted,
                value.hard,
                Callout::Error,
            ),
        ]
        .into_iter()
        .map(|(title, val, total, color_combo)| {
            StatState::get_gauge(title, val, total, color_combo)
        })
        .collect()
    }
}
