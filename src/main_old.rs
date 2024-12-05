use color_eyre::Result;
use std::env;
use std::time::Duration;
use crate::ui::screen::Screen;


fn main_old() -> Result<()> {
    color_eyre::install()?;
    // println!("HI!");
    // thread::sleep(Duration::from_secs(3));

    // let client = APIClient {
    //     backend: "http://localhost:8000/battle",
    //     player_id: 1,
    //     client: Client::new(),
    // };
    // runner(&client);
    // run();
    let player_id = env::args()
        .nth(1)
        .unwrap_or("1".to_string())
        .parse::<u16>()?;
    let mut app = Screen::new(player_id);
    app.run(Duration::from_millis(500));

    // _ = running_battle(&client);
    // loop {
    //     let battlefield = client.get_battlefield();
    //     match battlefield {
    //         None => {
    //             let result = client.get_result();
    //             match result {
    //                 None => {}
    //                 Some(br) => {
    //                     let ui = BattleResultUI {
    //                         win: br.hero_result.win,
    //                     };
    //                     terminal
    //                         .draw(|frame| {
    //                             ui.draw(frame);
    //                         })
    //                         .unwrap();
    //                 }
    //             }
    //         }
    //         Some(bf) => {
    //             terminal
    //                 .draw(|frame| {
    //                     frame.render_widget(&bf, frame.area());
    //                 })
    //                 .unwrap();
    //         }
    //     }
    //
    //     if let Ok(ok) = event::poll(interval) {
    //         if ok == false {
    //             continue;
    //         }
    //         let key = event::read().unwrap();
    //         let mut char_sent = None;
    //         match key {
    //             Event::Key(key) => {
    //                 if key.kind == KeyEventKind::Release {
    //                 } else if key.code == KeyCode::Char('q') {
    //                     break;
    //                 } else if key.code == KeyCode::Char('a') {
    //                     if client.press_key('a') {
    //                         char_sent = Some('a');
    //                     }
    //                 } else if key.code == KeyCode::Char('b') {
    //                     if client.press_key('b') {
    //                         char_sent = Some('b');
    //                     }
    //                 }
    //             }
    //             _ => {}
    //         }
    //         match char_sent {
    //             None => {}
    //             Some(_sent) => {}
    //         }
    //     }
    // }
    ratatui::restore();
    return Ok(());
}
