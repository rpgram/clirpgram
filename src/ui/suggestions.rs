use crate::entities::api::models::Suggestion;

fn button(back_button_key: u8) -> String {
    match back_button_key {
        1 => "a".to_string(),
        2 => "s".to_string(),
        3 => "d".to_string(),
        4 => "f".to_string(),
        _ => {
            panic!("Wrong API version?")
        }
    }
}

pub fn suggestion(suggestion: &Suggestion) -> String {
    format!(
        "Press {} to reach {} in {} turns",
        button(suggestion.button),
        suggestion.action,
        suggestion.steps_left
    )
}
