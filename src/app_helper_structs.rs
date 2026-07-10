pub use crate::anilist::anilist_types::{
    MediaFormat, MediaListSort, MediaListStatus, MediaSeason, MediaSort, MediaStatus, MediaType,
};
use crate::{
    anilist::anilist_types::{
        get_media, get_media_details,
        get_user_media_list::{self},
        update_entry,
    },
    utils::Utils,
};

use ratatui::widgets::TableState;
use std::collections::HashMap;

#[derive(PartialEq)]
pub enum ActiveBlock {
    Sidebar,
    Center,
    Details,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum CurrentView {
    UserAnime,
    UserManga,
    BrowseAnime,
    BrowseManga,
}

impl CurrentView {
    pub const ALL: [CurrentView; 4] = [
        CurrentView::UserAnime,
        CurrentView::UserManga,
        CurrentView::BrowseAnime,
        CurrentView::BrowseManga,
    ];
}
impl std::fmt::Display for CurrentView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            CurrentView::UserAnime => "Your Anime",
            CurrentView::UserManga => "Your Manga",
            CurrentView::BrowseAnime => "Browse Anime",
            CurrentView::BrowseManga => "Browse Manga",
        };
        write!(f, "{}", s)
    }
}

#[derive(Clone)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub allows_nsfw: Option<bool>,
}

impl User {
    pub fn get_name(&self) -> &str {
        &self.name
    }
}
pub struct PageInfo {
    pub current_page: i64,
    pub per_page: i64,
    pub total: Option<i64>,
    pub last_page: Option<i64>,
    pub has_next_page: Option<bool>,
}

pub struct UserMediaList {
    pub page_info: PageInfo,
    pub items: Option<Vec<MediaListItem>>,
}

impl From<get_media::ResponseData> for UserMediaList {
    fn from(data: get_media::ResponseData) -> Self {
        let mut page_info = PageInfo {
            current_page: 1,
            per_page: 50,
            total: None,
            last_page: None,
            has_next_page: None,
        };
        let mut items = Vec::new();

        if let Some(page) = data.page {
            if let Some(pi) = page.page_info {
                page_info.current_page = pi.current_page.unwrap_or(1);
                page_info.per_page = pi.per_page.unwrap_or(50);
                page_info.total = pi.total;
                page_info.last_page = pi.last_page;
                page_info.has_next_page = pi.has_next_page;
            }

            if let Some(media_array) = page.media {
                for m in media_array.into_iter().flatten() {
                    let id = m.id;

                    let titles = if let Some(t) = m.title.as_ref() {
                        Titles {
                            user_preferred: t
                                .user_preferred
                                .clone()
                                .unwrap_or_else(|| "Unknown".to_string()),
                            romaji: t.romaji.clone().unwrap_or_default(),
                            english: t.english.clone().unwrap_or_default(),
                            native: t.native.clone().unwrap_or_default(),
                        }
                    } else {
                        Titles {
                            user_preferred: "Unknown".to_string(),
                            romaji: "".to_string(),
                            english: "".to_string(),
                            native: "".to_string(),
                        }
                    };

                    let type_ = m.type_.unwrap_or(MediaType::Unknown);
                    let total = m.episodes.or(m.chapters);
                    let next_episode =
                        m.next_airing_episode
                            .as_ref()
                            .map(|airing| NextAiringEpisode {
                                time_until_airing: airing.time_until_airing,
                                episode: airing.episode,
                            });
                    let average_score = m.average_score;
                    let is_favourite = m.is_favourite;
                    let format = m.format;

                    items.push(MediaListItem {
                        id,
                        titles,
                        progress: None,
                        total,
                        type_,
                        average_score,
                        status: None,
                        next_airing_episode: next_episode,
                        format,
                        is_favourite,
                    });
                }
            }
        }

        UserMediaList {
            page_info,
            items: Some(items),
        }
    }
}

impl From<get_user_media_list::ResponseData> for UserMediaList {
    fn from(data: get_user_media_list::ResponseData) -> Self {
        let mut page_info = PageInfo {
            current_page: 1,
            per_page: 50,
            total: None,
            last_page: None,
            has_next_page: None,
        };
        let mut items = Vec::new();

        if let Some(page) = data.page {
            if let Some(pi) = page.page_info {
                page_info.current_page = pi.current_page.unwrap_or(1);
                page_info.per_page = pi.per_page.unwrap_or(50);
                page_info.total = pi.total;
                page_info.last_page = pi.last_page;
                page_info.has_next_page = pi.has_next_page;
            }

            if let Some(media_list) = page.media_list {
                for m in media_list.into_iter().flatten() {
                    let id = m.media.as_ref().map(|x| x.id).unwrap_or(0);
                    let titles = if let Some(t) = m.media.as_ref().and_then(|x| x.title.as_ref()) {
                        Titles {
                            user_preferred: t
                                .user_preferred
                                .clone()
                                .unwrap_or_else(|| "Unknown".to_string()),
                            romaji: t.romaji.clone().unwrap_or_default(),
                            english: t.english.clone().unwrap_or_default(),
                            native: t.native.clone().unwrap_or_default(),
                        }
                    } else {
                        Titles {
                            user_preferred: "Unknown".to_string(),
                            romaji: "".to_string(),
                            english: "".to_string(),
                            native: "".to_string(),
                        }
                    };

                    let mut total = None;
                    let type_ = m
                        .media
                        .as_ref()
                        .and_then(|med| med.type_)
                        .unwrap_or(MediaType::Unknown);

                    if let Some(m) = &m.media {
                        total = m.episodes.or(m.chapters);
                    };

                    let next_episode = m
                        .media
                        .as_ref()
                        .and_then(|next| next.next_airing_episode.clone())
                        .map(|airing| NextAiringEpisode {
                            time_until_airing: airing.time_until_airing,
                            episode: airing.episode,
                        });
                    let mapped_status: Option<MediaListStatus> = m.status;
                    let format = m.media.as_ref().and_then(|m| m.format);
                    let is_favourite = m
                        .media
                        .as_ref()
                        .map(|med| med.is_favourite)
                        .unwrap_or(false);

                    items.push(MediaListItem {
                        id,
                        titles,
                        progress: m.progress,
                        total,
                        type_,
                        average_score: None,
                        status: mapped_status,
                        next_airing_episode: next_episode,
                        format,
                        is_favourite,
                    });
                }
            }
        }

        UserMediaList {
            page_info,
            items: Some(items),
        }
    }
}
#[derive(Clone, Debug)]
pub struct NextAiringEpisode {
    pub episode: i64,
    pub time_until_airing: i64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TitleLanguage {
    UserPreferred,
    Romaji,
    English,
    Native,
}

impl TitleLanguage {
    pub const ALL: [TitleLanguage; 4] = [
        TitleLanguage::UserPreferred,
        TitleLanguage::Romaji,
        TitleLanguage::English,
        TitleLanguage::Native,
    ];

    pub fn to_string(self) -> &'static str {
        match self {
            TitleLanguage::UserPreferred => "User Preferred",
            TitleLanguage::Romaji => "Romaji",
            TitleLanguage::English => "English",
            TitleLanguage::Native => "Native (Kanji)",
        }
    }
}

#[derive(Clone, Debug)]
pub struct Titles {
    pub user_preferred: String,
    pub romaji: String,
    pub english: String,
    pub native: String,
}

impl Titles {
    pub fn get_title(&self, language: &TitleLanguage) -> &str {
        match language {
            TitleLanguage::UserPreferred => &self.user_preferred,
            TitleLanguage::Romaji => {
                if !self.romaji.is_empty() {
                    &self.romaji
                } else {
                    &self.user_preferred
                }
            }
            TitleLanguage::English => {
                if !self.english.is_empty() {
                    &self.english
                } else {
                    &self.romaji
                }
            }
            TitleLanguage::Native => {
                if !self.native.is_empty() {
                    &self.native
                } else {
                    &self.romaji
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct MediaListItem {
    pub id: i64,
    pub titles: Titles,
    pub progress: Option<i64>,
    pub total: Option<i64>,
    pub status: Option<MediaListStatus>,
    pub average_score: Option<i64>,
    pub next_airing_episode: Option<NextAiringEpisode>,
    pub type_: MediaType,
    pub is_favourite: bool,
    pub format: Option<MediaFormat>,
}

#[derive(PartialEq, Clone, Copy, Eq, Hash)]
pub enum BrowseCategory {
    CategoryOne,
    CategoryTwo,
    CategoryThree,
    Search,
}

impl BrowseCategory {
    pub const ALL: [BrowseCategory; 4] = [
        BrowseCategory::CategoryOne,
        BrowseCategory::CategoryTwo,
        BrowseCategory::CategoryThree,
        BrowseCategory::Search,
    ];
    pub fn next(&self) -> Self {
        match self {
            BrowseCategory::CategoryOne => BrowseCategory::CategoryTwo,
            BrowseCategory::CategoryTwo => BrowseCategory::CategoryThree,
            BrowseCategory::CategoryThree => BrowseCategory::Search,
            BrowseCategory::Search => BrowseCategory::CategoryOne,
        }
    }
    pub fn previous(&self) -> Self {
        match self {
            BrowseCategory::CategoryOne => BrowseCategory::Search,
            BrowseCategory::CategoryTwo => BrowseCategory::CategoryOne,
            BrowseCategory::CategoryThree => BrowseCategory::CategoryTwo,
            BrowseCategory::Search => BrowseCategory::CategoryThree,
        }
    }
}
impl BrowseCategory {
    pub fn to_string_user_anime(self) -> &'static str {
        match self {
            BrowseCategory::CategoryOne => "Watching",
            BrowseCategory::CategoryTwo => "Watched",
            BrowseCategory::CategoryThree => "Planning",
            BrowseCategory::Search => "All",
        }
    }
    pub fn to_string_user_manga(self) -> &'static str {
        match self {
            BrowseCategory::CategoryOne => "Reading",
            BrowseCategory::CategoryTwo => "Read",
            BrowseCategory::CategoryThree => "Planning",
            BrowseCategory::Search => "All",
        }
    }

    pub fn to_string_browse_anime(self) -> &'static str {
        match self {
            BrowseCategory::CategoryOne => "Trending",
            BrowseCategory::CategoryTwo => "This Season",
            BrowseCategory::CategoryThree => "Next Season",
            BrowseCategory::Search => "Search",
        }
    }

    pub fn to_string_browse_manga(self) -> &'static str {
        match self {
            BrowseCategory::CategoryOne => "Trending",
            BrowseCategory::CategoryTwo => "All Time Popular",
            BrowseCategory::CategoryThree => "Top Manga",
            BrowseCategory::Search => "Search",
        }
    }
}

pub struct BrowseState {
    pub loaded_view: CurrentView,
    pub media: Option<UserMediaList>,
    pub state: TableState,
    pub current_category: BrowseCategory,
    pub active_filters: HashMap<(CurrentView, BrowseCategory), SearchFilter>,
    pub active_user_filters: HashMap<(CurrentView, BrowseCategory), UserSearchFilter>,
}

#[derive(Clone, Copy)]
pub struct Date {
    pub year: Option<i64>,
    pub month: Option<i64>,
    pub day: Option<i64>,
}
impl std::fmt::Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match (self.year, self.month, self.day) {
            (Some(y), Some(m), Some(d)) => format!("{:04}-{:02}-{:02}", y, m, d),
            (Some(y), Some(m), None) => format!("{:04}-{:02}-??", y, m),
            (Some(y), None, None) => format!("{}", y),
            _ => "Unknown".to_string(),
        };
        write!(f, "{}", s)
    }
}
impl Date {
    pub fn empty() -> Date {
        Date {
            year: None,
            month: None,
            day: None,
        }
    }
    pub fn to_update_entry(self) -> update_entry::FuzzyDateInput {
        update_entry::FuzzyDateInput {
            year: self.year,
            month: self.month,
            day: self.day,
        }
    }
}

#[derive(Clone)]
pub struct UserMediaDetails {
    pub media_id: i64,
    pub user_media_id: Option<i64>,
    pub progress: i64,
    pub progress_volumes: Option<i64>,
    pub repeat: i64,
    pub started_at: Date,
    pub completed_at: Date,
    pub score: f64,
    pub status: MediaListStatus,
    pub notes: String,
}

pub struct MediaDetails {
    pub titles: Titles,
    pub description: String,
    pub average_score: i64,
    pub total: Option<i64>,
    pub volumes: Option<i64>,
    pub cover_image: String,
    pub season: MediaSeason,
    pub season_year: i64,
    pub site_url: String,
    pub media_status: MediaStatus,
    pub type_: MediaType,
    pub user_media_details: Option<UserMediaDetails>,
    pub start_date: Date,
    pub end_date: Date,
    pub is_favourite: bool,
    pub media_id: i64,
}

impl From<get_media_details::ResponseData> for MediaDetails {
    fn from(data: get_media_details::ResponseData) -> Self {
        let media = data.media;

        let titles = if let Some(t) = media.as_ref().and_then(|x| x.title.as_ref()) {
            Titles {
                user_preferred: t
                    .user_preferred
                    .clone()
                    .unwrap_or_else(|| "Unknown".to_string()),
                romaji: t.romaji.clone().unwrap_or_default(),
                english: t.english.clone().unwrap_or_default(),
                native: t.native.clone().unwrap_or_default(),
            }
        } else {
            Titles {
                user_preferred: "Unknown".to_string(),
                romaji: "".to_string(),
                english: "".to_string(),
                native: "".to_string(),
            }
        };

        let average_score = media.as_ref().and_then(|m| m.average_score).unwrap_or(0);

        let description = media
            .as_ref()
            .and_then(|m| m.description.clone())
            .unwrap_or_else(|| "No description available.".to_string())
            .replace("<br>", "\n");

        let total = media.as_ref().and_then(|m| m.chapters.or(m.episodes));
        let volumes = media.as_ref().and_then(|m| m.volumes);

        let cover_image = media
            .as_ref()
            .and_then(|m| m.cover_image.as_ref())
            .and_then(|c| c.large.clone())
            .unwrap_or_default();

        let season = media
            .as_ref()
            .and_then(|m| m.season)
            .unwrap_or(MediaSeason::Unknown);

        let type_ = media
            .as_ref()
            .and_then(|m| m.type_)
            .unwrap_or(MediaType::Unknown);

        let season_year = media.as_ref().and_then(|m| m.season_year).unwrap_or(0);

        let site_url = media
            .as_ref()
            .and_then(|m| m.site_url.clone())
            .unwrap_or_default();

        let start_date = media
            .as_ref()
            .and_then(|m| m.start_date.as_ref())
            .map(|d| Date {
                year: d.year,
                month: d.month,
                day: d.day,
            })
            .unwrap_or(Date::empty());

        let end_date = media
            .as_ref()
            .and_then(|m| m.end_date.as_ref())
            .map(|d| Date {
                year: d.year,
                month: d.month,
                day: d.day,
            })
            .unwrap_or(Date::empty());

        let is_favourite = media.as_ref().map(|m| m.is_favourite).unwrap_or(false);

        let media_status = media
            .as_ref()
            .and_then(|m| m.status)
            .unwrap_or(MediaStatus::Unknown);

        let media_id = media.as_ref().map(|m| m.id).unwrap_or(0);

        let mut user_media_details = None;
        if let Some(m) = media.as_ref().and_then(|m| m.media_list_entry.as_ref()) {
            let user_media_id = Some(m.id);
            let media_id = m.media_id;
            let score = m.score.unwrap_or(0.0);
            let progress = m.progress.unwrap_or(0);
            let status = m.status.unwrap_or(MediaListStatus::Unknown);
            let progress_volumes = m.progress_volumes;
            let repeat = m.repeat.unwrap_or(0);

            let started_at = m
                .started_at
                .as_ref()
                .map(|d| Date {
                    year: d.year,
                    month: d.month,
                    day: d.day,
                })
                .unwrap_or(Date::empty());

            let completed_at = m
                .completed_at
                .as_ref()
                .map(|d| Date {
                    year: d.year,
                    month: d.month,
                    day: d.day,
                })
                .unwrap_or(Date::empty());

            let notes = m.notes.clone().unwrap_or(String::new());

            user_media_details = Some(UserMediaDetails {
                user_media_id,
                media_id,
                score,
                progress,
                progress_volumes,
                status,
                repeat,
                started_at,
                completed_at,
                notes,
            });
        }

        MediaDetails {
            titles,
            description,
            average_score,
            total,
            volumes,
            type_,
            cover_image,
            season,
            season_year,
            site_url,
            media_status,
            user_media_details,
            start_date,
            end_date,
            is_favourite,
            media_id,
        }
    }
}

#[derive(Clone, Copy)]
pub enum CurrentEditField {
    Status,
    Score,
    EpisodeProgress,
    VolumeProgress,
    Rewatch,
    StartDate,
    EndDate,
    Notes,
}

pub enum ActivePopup {
    TitleLanguage,
    Error,
    EditMedia,
    Favourite,
    DeleteMedia,
    SearchFilter,
}

#[derive(Clone, Debug)]
pub struct SearchFilter {
    pub season: Option<MediaSeason>,
    pub year: Option<i64>,
    pub format: Option<MediaFormat>,
    pub status: Option<MediaStatus>,
    pub search: Option<String>,
    pub sort: Option<Vec<Option<MediaSort>>>,
}

impl SearchFilter {
    pub fn empty() -> Self {
        Self {
            season: None,
            year: None,
            format: None,
            status: None,
            search: None,
            sort: None,
        }
    }
    pub fn default_for(category: BrowseCategory, view: CurrentView) -> Self {
        match view {
            CurrentView::BrowseAnime => match category {
                BrowseCategory::CategoryOne => Self {
                    sort: Some(vec![Some(MediaSort::TrendingDesc)]),
                    ..Self::empty()
                },
                BrowseCategory::CategoryTwo => Self {
                    sort: Some(vec![Some(MediaSort::PopularityDesc)]),
                    season: Some(Utils::get_season()),
                    year: Some(Utils::get_year()),
                    ..Self::empty()
                },
                BrowseCategory::CategoryThree => Self {
                    sort: Some(vec![Some(MediaSort::PopularityDesc)]),
                    season: Some(Utils::get_season().next()),
                    year: Some(Utils::get_year()),
                    ..Self::empty()
                },
                BrowseCategory::Search => Self {
                    sort: Some(vec![Some(MediaSort::PopularityDesc)]),
                    ..Self::empty()
                },
            },
            CurrentView::BrowseManga => match category {
                BrowseCategory::CategoryOne => Self {
                    sort: Some(vec![Some(MediaSort::TrendingDesc)]),
                    ..Self::empty()
                },
                BrowseCategory::CategoryTwo => Self {
                    sort: Some(vec![Some(MediaSort::PopularityDesc)]),
                    ..Self::empty()
                },
                BrowseCategory::CategoryThree => Self {
                    sort: Some(vec![Some(MediaSort::ScoreDesc)]),
                    ..Self::empty()
                },
                BrowseCategory::Search => Self { ..Self::empty() },
            },
            _ => Self::empty(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct UserSearchFilter {
    pub sort: Option<Vec<MediaListSort>>,
    pub format: Option<MediaFormat>,
    pub status: Option<MediaListStatus>,
    pub favourites_only: bool,
}

impl UserSearchFilter {
    pub fn empty() -> Self {
        Self {
            format: None,
            status: None,
            sort: None,
            favourites_only: false,
        }
    }
    pub fn default_for(category: BrowseCategory, view: CurrentView) -> Self {
        match view {
            CurrentView::UserAnime => match category {
                BrowseCategory::CategoryOne => Self {
                    status: Some(MediaListStatus::Current),
                    sort: Some(vec![MediaListSort::UpdatedTimeDesc]),
                    ..Self::empty()
                },
                BrowseCategory::CategoryTwo => Self {
                    status: Some(MediaListStatus::Completed),
                    sort: Some(vec![MediaListSort::UpdatedTimeDesc]),
                    ..Self::empty()
                },
                BrowseCategory::CategoryThree => Self {
                    status: Some(MediaListStatus::Planning),
                    sort: Some(vec![MediaListSort::UpdatedTimeDesc]),
                    ..Self::empty()
                },
                BrowseCategory::Search => Self { ..Self::empty() },
            },
            CurrentView::UserManga => match category {
                BrowseCategory::CategoryOne => Self {
                    status: Some(MediaListStatus::Current),
                    sort: Some(vec![MediaListSort::UpdatedTimeDesc]),
                    ..Self::empty()
                },
                BrowseCategory::CategoryTwo => Self {
                    status: Some(MediaListStatus::Completed),
                    sort: Some(vec![MediaListSort::UpdatedTimeDesc]),
                    ..Self::empty()
                },
                BrowseCategory::CategoryThree => Self {
                    status: Some(MediaListStatus::Planning),
                    sort: Some(vec![MediaListSort::UpdatedTimeDesc]),
                    ..Self::empty()
                },
                BrowseCategory::Search => Self { 
                    sort: Some(vec![MediaListSort::UpdatedTimeDesc]),
                    ..Self::empty() },
            },
            _ => Self::empty(),
        }
    }
}

pub fn cycle_option<T: Clone + PartialEq>(current: &Option<T>, all: &[T], step: i32) -> Option<T> {
    if all.is_empty() {
        return None;
    }
    match current {
        None => {
            if step > 0 {
                Some(all[0].clone())
            } else {
                Some(all[all.len() - 1].clone())
            }
        }
        Some(val) => {
            let idx = all.iter().position(|x| x == val);
            match idx {
                None => None,
                Some(i) => {
                    let next_i = i as i32 + step;
                    if next_i < 0 || next_i >= all.len() as i32 {
                        None
                    } else {
                        Some(all[next_i as usize].clone())
                    }
                }
            }
        }
    }
}
