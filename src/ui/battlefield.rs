use crate::ui::moves::moved;
use crate::ui::players::player_state;
use crate::ui::results::acted;
use crate::ui::suggestions::suggestion;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::prelude::Color;
use ratatui::style::{Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets;
use ratatui::widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph, Widget};
use serde::Deserialize;
use crate::entities::api::models::{BattleEvent, Player, Suggestion};
use crate::entities::types::BattleId;

impl BattleField {
    fn render_player(player: &Player, area: Rect, buf: &mut Buffer) {
        let effects = &player.effects;
        let hp = player.health_points.to_string();
        let hp_status = Line::from(vec![
            Span::styled(hp, Style::new().green().bold()),
            Span::raw("    "),
            Span::styled(player_state(&player.state), Style::new().blue().italic()),
        ]);
        let delimiter = Line::default().fg(Color::Cyan);
        let mut text = vec![hp_status, delimiter];
        for effect in effects {
            text.push(Line::styled(effect, Style::new().yellow().italic()))
        }
        let username = Line::raw(&player.username);
        Paragraph::new(text)
            .block(
                Block::new()
                    .title(username)
                    .borders(Borders::BOTTOM | Borders::TOP)
                    .border_type(BorderType::Rounded),
            )
            .render(area, buf);
    }

    fn render_hints(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(Line::raw("You can:").centered())
            .borders(Borders::LEFT);

        let items: Vec<ListItem> = self
            .next_move
            .iter()
            .map(|a| ListItem::from(suggestion(a)).bg(Color::LightGreen))
            .collect::<Vec<ListItem>>();
        // .bg(NORMAL_ROW_BG);
        let list = List::new(items).block(block);
        widgets::StatefulWidget::render(list, area, buf, &mut ListState::default())
    }

    fn render_moves(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(Line::raw("Moved:").centered())
            .borders(Borders::RIGHT);

        let items: Vec<ListItem> = self
            .moves
            .iter()
            .map(|a| ListItem::from(moved(*a)))
            .collect::<Vec<ListItem>>();
        // .bg(NORMAL_ROW_BG);
        let list = List::new(items).block(block);
        widgets::StatefulWidget::render(list, area, buf, &mut ListState::default())
    }

    fn render_actions(&self, area: Rect, buf: &mut Buffer) {
        let mut actions = vec![];
        for act in &self.complete_actions {
            let space = Span::raw("          ");
            let action_line;
            if act.side == 1 {
                let acted_span = Span::styled(acted(act.action), Style::new().green());
                action_line = Line::from(vec![acted_span, space]);
            } else {
                let acted_span = Span::styled(acted(act.action), Style::new().red());
                action_line = Line::from(vec![space, acted_span]);
            }
            actions.push(action_line)
        }
        Paragraph::new(actions)
            .block(Block::new().title("Done!").borders(Borders::LEFT))
            .render(area, buf);
    }
}

impl Widget for &BattleField {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let [player_layout, hints, results, opponent] = Layout::horizontal([
            Constraint::Percentage(20),
            Constraint::Percentage(35),
            Constraint::Fill(1),
            Constraint::Percentage(20),
        ])
        .areas(area);
        let [player_profile, moves] =
            Layout::vertical([Constraint::Percentage(30), Constraint::Percentage(70)])
                .areas(player_layout);
        BattleField::render_player(&self.player, player_profile, buf);
        BattleField::render_player(&self.opponent, opponent, buf);
        self.render_hints(hints, buf);
        self.render_moves(moves, buf);
        self.render_actions(results, buf);
    }
}

#[derive(Deserialize)]
pub struct BattleField {
    pub battle_id: BattleId,
    player: Player,
    opponent: Player,
    next_move: Vec<Suggestion>,
    complete_actions: Vec<BattleEvent>,
    moves: Vec<u8>,
}
