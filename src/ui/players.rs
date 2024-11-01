pub fn player_state(player_state: &u8) -> String {
    match player_state {
        0 => "Alive".to_string(),
        1 => "Muted".to_string(),
        2 => "Preparation".to_string(),
        _ => {
            panic!("Wrong Backend version?")
        }
    }
}
