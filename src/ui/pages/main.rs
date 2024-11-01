use crate::ui::pages::connect::ConnectMenu;
use crate::ui::pages::menu::{Action, get_action, IMenu, MenuTag, MenuWidget};
use crate::ui::pages::start::StartBattleMenu;
use crate::ui::screen::Screen;

#[derive(Clone)]
pub struct Menu {}



impl IMenu for Menu {
    fn choose(&self, screen: &mut Screen) {
        match get_action(screen) {
            Action::StartInput => {self.get_players_menu(screen)}
            Action::StartBattle(_) => {}
            Action::ConnectToBattle => {self.get_waiting_battles_menu(screen)}
            Action::ChooseToConnect(_) => {}
        }
        // let idx = screen.current_widget.state().selected().unwrap_or(0);
        // let choice = screen.current_widget.chosen(idx);
        // let new_state = self.handle_action(choice.action, screen);
        // screen.current_menu = Box::new(new_state);
    }

    fn self_type(&self) -> MenuTag {
        MenuTag::MainMenu
    }
}

// impl Widget for &mut Menu {
//     fn render(self, area: Rect, buf: &mut Buffer)
//     where
//         Self: Sized,
//     {
//         render_menu(&self.choices, &mut self.menu_state, area, buf);
//     }
// }

impl Menu {
    // pub fn factory(idx: Option<usize>) -> Self {
    //     let choices = vec![
    //         MenuChoice {
    //             action: Action::StartInput,
    //             title: "Begin".to_string(),
    //         },
    //         MenuChoice {
    //             action: Action::ConnectToBattle,
    //             title: "Connect".to_string(),
    //         },
    //     ];
    //     let mut mm = Self {
    //         menu_widget: MenuWidget {
    //             choices,
    //             menu_state: ListState::default(),
    //             injected_menu: None,
    //         },
    // };
    // let link = Rc::new(&mut mm);
    // mm.menu_widget.injected_menu = Some(link);
    // mm.menu_widget.menu_state.select(idx);
    // mm
    // }

    // fn handle_action(&self, action: Action, screen: &mut Screen) {
    //     match action {
    //         // Action::ConnectToBattle => self.get_battles_menu(screen),
    //         Action::StartInput => self.get_players_menu(screen),
    //         Action::ConnectToBattle => self.get_waiting_battles_menu(screen),
    //         _ => {
    //             panic!()
    //         }
    //     }
    // }

    fn get_battles_menu(&self, _screen: &Screen) -> ConnectMenu {
        panic!()
        // let mut choices = vec![];
        // let mut battle_ids = vec![];
        // for waiting_battle in screen.api_client.get_waiting_battles() {
        //     let title = format!("User {} - {}", waiting_battle.player_id.clone(), waiting_battle.battle_id);
        //     battle_ids.push(waiting_battle.battle_id);
        // }
        // ConnectMenu {choices:self.choices.clone(), menu_state}
    }

    fn get_players_menu(&self, screen: &mut Screen) {
        screen.current_widget = MenuWidget::start(screen);
        screen.current_menu = Box::new(StartBattleMenu {})
    }

    fn get_waiting_battles_menu(&self, screen: &mut Screen) {
        screen.current_widget = MenuWidget::connect(screen);
        screen.current_menu = Box::new(ConnectMenu {})
    }
}
