use crate::ui::pages::main::Menu;
use crate::pages::interface::IMenu;
use crate::ui::pages::menu::{scroll_down, scroll_up, IMenuWidget, MenuWidget};
use ratatui::crossterm::event;
use ratatui::crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::DefaultTerminal;
use reqwest::blocking::Client;
use std::time::Duration;
use crate::entities::types::PlayerId;
use crate::shared::api::client::APIClient;

pub struct Screen<'a> {
    pub current_menu: Box<dyn IMenu>,
    pub current_widget: MenuWidget,
    pub api_client: APIClient<'a>,
    pub terminal: DefaultTerminal,
}

impl Screen<'_> {
    pub fn new(player_id: PlayerId) -> Self {
        let client = APIClient {
            backend: "http://localhost:8000/battle",
            player_id,
            client: Client::new(),
        };
        let terminal = ratatui::init();
        Self {
            current_widget: MenuWidget::main(),
            current_menu: Box::new(Menu {}),
            api_client: client,
            // current_widget: MenuWidget {},
            terminal,
        }
    }

    pub fn run(&mut self, interval: Duration) {
        // let mut current_menu = MenuTag::MainMenu(0);
        loop {
            self.terminal
                .draw(|frame| frame.render_widget(&mut self.current_widget, frame.area()))
                .unwrap();

            if let Ok(ok) = event::poll(interval) {
                if ok == false {
                    continue;
                }
                let key = event::read().unwrap();
                match key {
                    Event::Key(k) => self.handle_key(k),
                    _ => {}
                }
            }
        }
    }
    fn handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        let current_menu = dyn_clone::clone(&self.current_menu);
        // let current_menu = Box::clone(&self.current_menu);
        match key.code {
            KeyCode::Down => {
                scroll_down(self.current_widget.state());
            }
            KeyCode::Up => {
                scroll_up(self.current_widget.state());
            }
            KeyCode::Enter => {
                current_menu.choose(self);
            }
            _ => {
                panic!()
            }
        }
    }
}
