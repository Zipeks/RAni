use graphql_client::GraphQLQuery;
use serde::{Deserialize, Serialize};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "qraphql/get_media_details.graphql",
    response_derives = "Debug,Clone",
    extern_enums(
        "MediaType",
        "MediaSort",
        "MediaStatus",
        "MediaSeason",
        "MediaFormat",
        "MediaListStatus"
    ),
    skip_serializing_none
)]
pub struct GetMediaDetails;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "qraphql/get_basic_viewer.graphql",
    response_derives = "Debug",
    skip_serializing_none
)]
pub struct GetBasicViewer;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "qraphql/get_user_media_list.graphql",
    response_derives = "Debug, Clone",
    extern_enums(
        "MediaType",
        "MediaListSort",
        "MediaListStatus",
        "MediaFormat",
        // "MediaSeason"
    ),
    skip_serializing_none
)]
pub struct GetUserMediaList;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "qraphql/get_media.graphql",
    response_derives = "Debug, Clone",
    extern_enums("MediaType", "MediaSort", "MediaStatus", "MediaSeason", "MediaFormat"),
    skip_serializing_none
)]
pub struct GetMedia;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "qraphql/update_entry.graphql",
    response_derives = "Debug, Clone",
    extern_enums("MediaListStatus"),
    skip_serializing_none
)]
pub struct UpdateEntry;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "qraphql/toggle_favourite.graphql",
    response_derives = "Debug, Clone",
    skip_serializing_none
)]
pub struct ToggleFavourite;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "qraphql/delete_media_list_entry.graphql",
    response_derives = "Debug, Clone",
    skip_serializing_none
)]
pub struct DeleteMediaListEntry;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MediaType {
    Anime,
    Manga,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MediaListStatus {
    Current,
    Planning,
    Completed,
    Repeating,
    Dropped,
    Paused,
    #[serde(other)]
    Unknown,
}

impl MediaListStatus {
    pub const ALL: [MediaListStatus; 6] = [
        MediaListStatus::Planning,
        MediaListStatus::Current,
        MediaListStatus::Completed,
        MediaListStatus::Dropped,
        MediaListStatus::Paused,
        MediaListStatus::Repeating,
    ];
    pub fn next(&self) -> Self {
        let index = Self::ALL.iter().position(|x| x == self).unwrap_or(0);
        Self::ALL[(index + 1) % Self::ALL.len()]
    }

    pub fn previous(&self) -> Self {
        let index = Self::ALL.iter().position(|x| x == self).unwrap_or(0);
        Self::ALL[(index + Self::ALL.len() - 1) % Self::ALL.len()]
    }
}

impl std::fmt::Display for MediaListStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            MediaListStatus::Current => "Current",
            MediaListStatus::Planning => "Planning",
            MediaListStatus::Completed => "Completed",
            MediaListStatus::Dropped => "Dropped",
            MediaListStatus::Paused => "Paused",
            MediaListStatus::Repeating => "Repeating",
            MediaListStatus::Unknown => "Unknown",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MediaStatus {
    Finished,
    Releasing,
    NotYetReleased,
    Cancelled,
    Hiatus,
    #[serde(other)]
    Unknown,
}

impl MediaStatus {
    pub const ALL: [MediaStatus; 5] = [
        MediaStatus::Finished,
        MediaStatus::Releasing,
        MediaStatus::NotYetReleased,
        MediaStatus::Cancelled,
        MediaStatus::Hiatus,
    ];
    pub fn next(self) -> Self {
        let index = Self::ALL.iter().position(|x| x == &self).unwrap_or(0);
        Self::ALL[(index + 1) % Self::ALL.len()]
    }
    pub fn previous(self) -> Self {
        let index = Self::ALL.iter().position(|x| x == &self).unwrap_or(0);
        Self::ALL[(index + Self::ALL.len() - 1) % Self::ALL.len()]
    }
}

impl std::fmt::Display for MediaStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            MediaStatus::Finished => "Finished",
            MediaStatus::Releasing => "Releasing",
            MediaStatus::NotYetReleased => "Not yet released",
            MediaStatus::Cancelled => "Cancelled",
            MediaStatus::Hiatus => "Hiatus",
            MediaStatus::Unknown => "Unknown",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MediaSeason {
    Winter,
    Spring,
    Summer,
    Fall,
    #[serde(other)]
    Unknown,
}

impl MediaSeason {
    pub const ALL: [MediaSeason; 4] = [
        MediaSeason::Winter,
        MediaSeason::Spring,
        MediaSeason::Summer,
        MediaSeason::Fall,
    ];
    pub fn next(&self) -> Self {
        match self {
            MediaSeason::Winter => MediaSeason::Spring,
            MediaSeason::Spring => MediaSeason::Summer,
            MediaSeason::Summer => MediaSeason::Fall,
            MediaSeason::Fall => MediaSeason::Winter,
            MediaSeason::Unknown => MediaSeason::Unknown,
        }
    }
}

impl std::fmt::Display for MediaSeason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            MediaSeason::Winter => "Winter",
            MediaSeason::Spring => "Spring",
            MediaSeason::Summer => "Summer",
            MediaSeason::Fall => "Fall",
            MediaSeason::Unknown => "Unknown",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MediaFormat {
    Tv,
    TvShort,
    Movie,
    Ova,
    Ona,
    Music,
    Manga,
    Novel,
    OneShot,
    #[serde(other)]
    Unknown,
}

impl MediaFormat {
    pub const ALL: [MediaFormat; 9] = [
        MediaFormat::Tv,
        MediaFormat::TvShort,
        MediaFormat::Movie,
        MediaFormat::Ova,
        MediaFormat::Ona,
        MediaFormat::Music,
        MediaFormat::Manga,
        MediaFormat::Novel,
        MediaFormat::OneShot,
    ];
    pub const ANIME: [MediaFormat; 6] = [
        MediaFormat::Tv,
        MediaFormat::TvShort,
        MediaFormat::Movie,
        MediaFormat::Ova,
        MediaFormat::Ona,
        MediaFormat::Music,
        // MediaFormat::OneShot,
    ];
    pub const MANGA: [MediaFormat; 3] =
        [MediaFormat::Manga, MediaFormat::Novel, MediaFormat::OneShot];
    pub fn next(self) -> Self {
        let index = Self::ALL.iter().position(|x| x == &self).unwrap_or(0);
        Self::ALL[(index + 1) % Self::ALL.len()]
    }
    pub fn previous(self) -> Self {
        let index = Self::ALL.iter().position(|x| x == &self).unwrap_or(0);
        Self::ALL[(index + Self::ALL.len() - 1) % Self::ALL.len()]
    }
}

impl std::fmt::Display for MediaFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            MediaFormat::Tv => "TV",
            MediaFormat::TvShort => "TV Short",
            MediaFormat::Movie => "Movie",
            MediaFormat::Ova => "OVA",
            MediaFormat::Ona => "ONA",
            MediaFormat::Music => "Music",
            MediaFormat::Manga => "Manga",
            MediaFormat::Novel => "Light Novel",
            MediaFormat::OneShot => "One Shot",
            MediaFormat::Unknown => "Unknown",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MediaListSort {
    MediaPopularityDesc,
    ScoreDesc,
    UpdatedTimeDesc,
    #[serde(other)]
    Unknown,
}

impl MediaListSort {
    pub const ALL: [MediaListSort; 3] = [
        MediaListSort::MediaPopularityDesc,
        MediaListSort::ScoreDesc,
        MediaListSort::UpdatedTimeDesc,
    ];
    pub fn next(self) -> Self {
        let index = Self::ALL.iter().position(|x| x == &self).unwrap_or(0);
        Self::ALL[(index + 1) % Self::ALL.len()]
    }
    pub fn previous(self) -> Self {
        let index = Self::ALL.iter().position(|x| x == &self).unwrap_or(0);
        Self::ALL[(index + Self::ALL.len() - 1) % Self::ALL.len()]
    }
}

impl std::fmt::Display for MediaListSort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            MediaListSort::MediaPopularityDesc => "Popularity",
            MediaListSort::ScoreDesc => "Score",
            MediaListSort::UpdatedTimeDesc => "Updated Time",
            MediaListSort::Unknown => "Other",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MediaSort {
    PopularityDesc,
    TrendingDesc,
    ScoreDesc,
    UpdatedTimeDesc,
    #[serde(other)]
    Unknown,
}

impl MediaSort {
    pub const ALL: [MediaSort; 3] = [
        MediaSort::PopularityDesc,
        MediaSort::TrendingDesc,
        MediaSort::ScoreDesc,
    ];
    pub fn next(self) -> Self {
        let index = Self::ALL.iter().position(|x| x == &self).unwrap_or(0);
        Self::ALL[(index + 1) % Self::ALL.len()]
    }
    pub fn previous(self) -> Self {
        let index = Self::ALL.iter().position(|x| x == &self).unwrap_or(0);
        Self::ALL[(index + Self::ALL.len() - 1) % Self::ALL.len()]
    }
}

impl std::fmt::Display for MediaSort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            MediaSort::PopularityDesc => "Popularity",
            MediaSort::TrendingDesc => "Trending",
            MediaSort::ScoreDesc => "Score",
            MediaSort::UpdatedTimeDesc => "Updated Time",
            MediaSort::Unknown => "Other",
        };
        write!(f, "{}", s)
    }
}
