use std::sync::mpsc::Receiver;
use ratatui::DefaultTerminal;
use crate::entities::models::menu::KeyToUI;
use crate::pages::interface::IMenu;
use crate::ui::pages::menu::MenuWidget;

pub struct UIController {

    pub current_menu: Box<dyn IMenu>,
    pub current_widget: MenuWidget,
    pub terminal: DefaultTerminal,

}

impl UIController {
    pub fn view (&mut self) {
        self.terminal
            .draw(|frame| frame.render_widget(&mut self.current_widget, frame.area()))
            .unwrap();



    }
}