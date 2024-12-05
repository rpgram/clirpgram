use dyn_clone::DynClone;
use crate::ui::pages::connect::ConnectMenu;
use crate::ui::pages::main::Menu;
use crate::entities::models::menu::MenuTag;
use crate::ui::pages::start::StartBattleMenu;
use crate::ui::screen::Screen;

pub trait IMenu: DynClone {
    fn choose(&self, screen: &mut Screen);

    fn self_type(&self) -> MenuTag;
}

impl Clone for Box<dyn IMenu> {
    fn clone(&self) -> Self {
        match self.self_type() {
            MenuTag::MainMenu => Box::new(Menu {}),
            MenuTag::ConnectMenu => Box::new(ConnectMenu {}),
            MenuTag::StartMenu => Box::new(StartBattleMenu {}),
        }
    }
}