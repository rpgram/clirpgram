use crate::ui::results::BattleResultUI;
use crate::ui::screen::Screen;
use ratatui::crossterm::event;
use ratatui::crossterm::event::{Event, KeyCode, KeyEventKind};
use std::time::Duration;

pub fn running_battle(screen: &mut Screen, interval: Duration) {
    // let battle = screen.api_client.get_battlefield();
    // match battle {
    //     None => None,
    //     Some(bf) => return Some(bf.battle_id),
    // }
    loop {
        let battlefield = screen.api_client.get_battlefield();
        match battlefield {
            None => {
                let result = screen.api_client.get_result();
                match result {
                    None => {}
                    Some(br) => {
                        let win;
                        if screen.api_client.player_id == br.hero_result.player_id {
                            win = br.hero_result.win
                        } else {
                            win = !br.hero_result.win
                        }

                        let ui = BattleResultUI { win };
                        screen
                            .terminal
                            .draw(|frame| {
                                ui.draw(frame);
                            })
                            .unwrap();
                    }
                }
            }
            Some(bf) => {
                screen
                    .terminal
                    .draw(|frame| {
                        frame.render_widget(&bf, frame.area());
                    })
                    .unwrap();
            }
        }
        //
        if let Ok(ok) = event::poll(interval) {
            if ok == false {
                continue;
            }
            let key = event::read().unwrap();
            let mut char_sent = None;
            match key {
                Event::Key(key) => {
                    if key.kind == KeyEventKind::Release {
                    } else if key.code == KeyCode::Char('q') {
                        break;
                    } else if key.code == KeyCode::Char('a') {
                        if screen.api_client.press_key('a') {
                            char_sent = Some('a');
                        }
                    } else if key.code == KeyCode::Char('b') {
                        if screen.api_client.press_key('b') {
                            char_sent = Some('b');
                        }
                    }
                }
                _ => {}
            }
            match char_sent {
                None => {}
                Some(_sent) => {}
            }
        }
    }
    // return aclient.start_battle(1);
}
