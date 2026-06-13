use crate::anilist::anilist_types::MediaSeason;
use chrono::Datelike;
pub struct Utils {}
impl Utils {
    pub fn get_year() -> i64 {
        chrono::Utc::now().year() as i64
    }
    pub fn get_season() -> MediaSeason {
        let current_date = chrono::Utc::now();
        let month = current_date.month();
        match month {
            1..=3 => MediaSeason::Winter,
            4..=6 => MediaSeason::Spring,
            7..=9 => MediaSeason::Summer,
            10..=12 => MediaSeason::Fall,
            _ => unimplemented!(),
        }
    }
}
