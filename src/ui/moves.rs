pub fn moved(action: u8) -> String {
    match action {
        0 => "Hit".to_string(),
        1 => "Thud".to_string(),
        2 => "Wham".to_string(),
        3 => "Beep".to_string(),
        4 => "Krik".to_string(),
        _ => panic!(),
    }
}
