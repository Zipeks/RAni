use crate::app_helper_structs::Season;
use chrono::Datelike;
pub struct Utils {}
impl Utils {
    pub fn get_year() -> i64 {
        chrono::Utc::now().year() as i64
    }
    pub fn get_season() -> Season {
        let current_date = chrono::Utc::now();
        let month = current_date.month();
        match month {
            1..=3 => Season::Winter,
            4..=6 => Season::Spring,
            7..=9 => Season::Summer,
            10..=12 => Season::Fall,
            _ => unimplemented!(),
        }
    }
}
