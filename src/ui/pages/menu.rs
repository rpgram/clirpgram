use crate::ui::screen::Screen;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Stylize;
use ratatui::text::Line;
use ratatui::widgets;
use ratatui::widgets::{HighlightSpacing, List, ListItem, ListState, Widget};
use crate::entities::models::menu::{Action, MenuChoice};
// pub enum Menus {
//     Main(Menu),
//     Start(StartBattleMenu),
//     Connect,
// }

// pub fn render_menu(
//     choices: &Vec<MenuChoice>,
//     menu_state: &mut ListState,
//     area: Rect,
//     buf: &mut Buffer,
// ) {
//     let mut items = vec![];
//     for c in choices {
//         items.push(ListItem::new(Line::raw(&c.title)).bold().blue())
//     }
//
//     let list = List::new(items);
//     widgets::StatefulWidget::render(list, area, buf, menu_state);
//     menu_state.select_first();
// }

pub trait IMenuWidget {
    fn render_menu(&mut self, area: Rect, buf: &mut Buffer);

    fn state(&mut self) -> &mut ListState;

    fn chosen(&self, idx: usize) -> MenuChoice;
}

#[derive(Clone)]
pub struct MenuWidget {
    pub choices: Vec<MenuChoice>,
    pub menu_state: ListState,
}

impl IMenuWidget for MenuWidget {
    fn render_menu(&mut self, area: Rect, buf: &mut Buffer) {
        let mut items = vec![];
        for c in &self.choices {
            items.push(ListItem::new(Line::raw(c.title.clone())).bold().blue())
        }

        let list = List::new(items)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::WhenSelected);
        widgets::StatefulWidget::render(list, area, buf, &mut self.menu_state);
    }

    fn state(&mut self) -> &mut ListState {
        &mut self.menu_state
    }

    fn chosen(&self, idx: usize) -> MenuChoice {
        return self.choices[idx].clone();
    }
}

impl Widget for &mut MenuWidget {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        self.render_menu(area, buf);
    }
}

impl MenuWidget {
    pub fn main() -> Self {
        let choices = vec![
            MenuChoice {
                action: Action::StartInput,
                title: "Begin".to_string(),
            },
            MenuChoice {
                action: Action::ConnectToBattle,
                title: "Connect".to_string(),
            },
        ];
        Self {
            choices,
            menu_state: ListState::default(),
        }
    }

    pub fn start(screen: &mut Screen) -> Self {
        let players = screen.api_client.get_players();
        let mut choices = vec![];
        for player in players.into_iter() {
            choices.push(MenuChoice {
                action: Action::StartBattle(Some(player.player_id)),
                title: player.username,
            })
        }
        choices.push(MenuChoice {
            action: Action::StartBattle(None),
            title: "Waiting".to_string(),
        });
        Self {
            choices,
            menu_state: ListState::default(),
        }
    }

    pub fn connect(screen: &mut Screen) -> Self {
        let battles = screen.api_client.get_waiting_battles();
        let mut choices = vec![];
        for battle in battles.into_iter() {
            choices.push(MenuChoice {
                action: Action::ChooseToConnect(battle.battle_id),
                title: format!("{} for {}", battle.player_id, battle.battle_id),
            })
        }
        Self {
            choices,
            menu_state: ListState::default(),
        }
    }
}

pub fn scroll_down(menu_state: &mut ListState) {
    menu_state.scroll_down_by(1);
}

pub fn scroll_up(menu_state: &mut ListState) {
    menu_state.scroll_up_by(1);
}

pub fn get_action(screen: &mut Screen) -> Action {
    let idx = screen.current_widget.state().selected().unwrap_or(0);
    let choice = screen.current_widget.chosen(idx);
    return choice.action;
}
