use crate::application::start_battle::running_battle;
use crate::ui::pages::menu::{get_action, Action, IMenu, MenuTag};
use crate::ui::screen::Screen;
use std::time::Duration;

#[derive(Clone)]
pub struct ConnectMenu {}

impl IMenu for ConnectMenu {
    fn choose(&self, screen: &mut Screen) {
        match get_action(screen) {
            Action::StartInput => {}
            Action::StartBattle(_) => {}
            Action::ConnectToBattle => {}
            Action::ChooseToConnect(battle_id) => {
                _ = screen.api_client.connect_to_battle(battle_id);
                running_battle(screen, Duration::from_millis(500));
                panic!()
            }
        }
        // let idx = screen.current_widget.state().selected().unwrap_or(0);
        // let choice = screen.current_widget.chosen(idx);
        // let new_state = self.handle_action(choice.action, screen);
        // screen.current_menu = Box::new(new_state);
    }

    fn self_type(&self) -> MenuTag {
        MenuTag::ConnectMenu
    }
}

// impl ConnectMenu {
//
//     fn handle_action(&self, action: Action, screen: &mut Screen) -> impl IMenu {
//         match action {
//             Action::ChooseToConnect(battle_id) => {
//                 _ = screen.api_client.connect_to_battle(battle_id);
//                 running_battle(screen, Duration::from_millis(500));
//                 panic!()
//             }
//             _ => {
//                 panic!()
//             }
//         }
//     }
// }
