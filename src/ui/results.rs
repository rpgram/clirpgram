use ratatui::prelude::Widget;
use ratatui::style::{Style, Stylize};
use ratatui::widgets::canvas::Canvas;
use ratatui::widgets::Block;
use ratatui::Frame;

pub fn acted(acted_id: u8) -> String {
    match acted_id {
        1 => "SWEEP".to_string(),
        2 => "FREEZE".to_string(),
        4 => "STUN".to_string(),
        0 => "HIT".to_string(),
        3 => "PIERCE".to_string(),
        _ => panic!("Wrong API version?"),
    }
}

pub struct BattleResultUI {
    pub win: bool,
}

impl BattleResultUI {
    pub fn draw(self, frame: &mut Frame)
    where
        Self: Sized,
    {
        let text;
        let style;
        if self.win {
            text = "YOU WIN!!!";
            style = Style::new().green().on_light_blue().bold();
        } else {
            text = "You lost.";
            style = Style::new().red().on_yellow().italic();
        }
        frame.render_widget(
            BattleResultUI::draw_result_text(text.to_string(), style),
            frame.area(),
        )
    }

    fn draw_result_text(text: String, style: Style) -> impl Widget {
        Canvas::default()
            .block(Block::bordered().style(style))
            .paint(move |ctx| ctx.print(0.0, 0.0, text.clone()))
            .x_bounds([-180.0, 180.0])
            .y_bounds([-90.0, 90.0])
    }
}
