use std::sync::mpsc::Receiver;
use ratatui::DefaultTerminal;
use crate::pages::interface::IMenu;
use crate::ui::pages::menu::MenuWidget;

pub struct UIController {

    pub current_menu: Box<dyn IMenu>,
    pub current_widget: MenuWidget,
    pub terminal: DefaultTerminal,

}

impl UIController {
    pub fn run (&mut self, rx:Receiver<>) {
        self.terminal
            .draw(|frame| frame.render_widget(&mut self.current_widget, frame.area()))
            .unwrap();



    }
}