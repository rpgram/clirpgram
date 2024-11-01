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
    }

    fn self_type(&self) -> MenuTag {
        MenuTag::MainMenu
    }
}

impl Menu {

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
