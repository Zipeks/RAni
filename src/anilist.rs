use graphql_client::{GraphQLQuery, Response};
use moka::future::Cache;
use reqwest::{Client, header};
use std::{error::Error, time::Duration};

use crate::app_helper_structs::{
    MediaFormat, MediaListSort, MediaListStatus, MediaSeason, MediaSort, MediaStatus, MediaType,
    SearchFilter, UserMediaDetails,
};

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
        "MediaSeason"
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

#[derive(Clone)]
pub struct AnilistClient {
    http_client: Client,
    api_url: &'static str,
    media_list_cache: Cache<String, String>,
    details_cache: Cache<i64, String>,
}

impl AnilistClient {
    pub fn new(access_token: Option<&str>) -> Result<Self, Box<dyn Error + Sync + Send>> {
        let mut headers = header::HeaderMap::new();
        if let Some(token) = access_token {
            let auth_value = format!("Bearer {}", token);
            let mut header_value = header::HeaderValue::from_str(&auth_value)?;

            header_value.set_sensitive(true);
            headers.insert(header::AUTHORIZATION, header_value);
        }
        let client = Client::builder().default_headers(headers).build()?;
        let media_list_cache = Cache::builder()
            .max_capacity(100)
            .time_to_live(Duration::from_secs(300))
            .build();
        let details_cache = Cache::builder()
            .max_capacity(100)
            .time_to_live(Duration::from_secs(300))
            .build();
        Ok(Self {
            http_client: client,
            api_url: "https://graphql.anilist.co",
            media_list_cache,
            details_cache,
        })
    }

    pub async fn get_basic_viewer(
        &self,
    ) -> Result<get_basic_viewer::ResponseData, Box<dyn Error + Sync + Send>> {
        let variables = get_basic_viewer::Variables;
        let request_body = GetBasicViewer::build_query(variables);

        let res = self
            .http_client
            .post(self.api_url)
            .json(&request_body)
            .send()
            .await?;

        let response_body: Response<get_basic_viewer::ResponseData> = res.json().await?;

        if let Some(errors) = response_body.errors {
            return Err(format!("GraphQL Error: {:?}", errors).into());
        }

        response_body.data.ok_or_else(|| "No data".into())
    }

    pub async fn get_user_media_list(
        &self,
        user_id: i64,
        status: Option<MediaListStatus>,
        sort: Option<Vec<Option<MediaListSort>>>,
        page: Option<i64>,
        per_page: Option<i64>,
        type_: MediaType,
    ) -> Result<get_user_media_list::ResponseData, Box<dyn std::error::Error + Sync + Send>> {
        let variables = get_user_media_list::Variables {
            user_id,
            status,
            sort,
            page,
            per_page,
            type_,
        };

        let request_body = GetUserMediaList::build_query(variables);

        let json_body = serde_json::to_value(&request_body)?;

        let cache_key = json_body.to_string();

        if let Some(cached_response) = self.media_list_cache.get(&cache_key).await {
            let response_body: graphql_client::Response<get_user_media_list::ResponseData> =
                serde_json::from_str(&cached_response)?;

            if let Some(errors) = response_body.errors {
                return Err(format!("GraphQL Error: {:?}", errors).into());
            }
            return response_body.data.ok_or_else(|| "No data".into());
        }

        let res = self
            .http_client
            .post(self.api_url)
            .json(&json_body)
            .send()
            .await?;

        let raw_response_text = res.text().await?;

        self.media_list_cache
            .insert(cache_key, raw_response_text.clone())
            .await;

        let response_body: graphql_client::Response<get_user_media_list::ResponseData> =
            serde_json::from_str(&raw_response_text)?;

        if let Some(errors) = response_body.errors {
            return Err(format!("GraphQL Error: {:?}", errors).into());
        }

        response_body.data.ok_or_else(|| "No data".into())
    }

    pub async fn get_media(
        &self,
        type_: MediaType,
        search_filter: SearchFilter,
        page: Option<i64>,
        per_page: Option<i64>,
    ) -> Result<get_media::ResponseData, Box<dyn std::error::Error + Sync + Send>> {
        let clean_search = search_filter.search.filter(|s| !s.trim().is_empty());

        let variables = get_media::Variables {
            season: search_filter.season,
            season_year: search_filter.year,
            status: search_filter.media_status,
            sort: search_filter.sort,
            page,
            per_page,
            type_,
            search: clean_search,
            format: search_filter.format,
        };

        let request_body = GetMedia::build_query(variables);

        let json_body = serde_json::to_value(&request_body)?;

        let cache_key = json_body.to_string();

        if let Some(cached_response) = self.media_list_cache.get(&cache_key).await {
            let response_body: graphql_client::Response<get_media::ResponseData> =
                serde_json::from_str(&cached_response)?;

            if let Some(errors) = response_body.errors {
                return Err(format!("GraphQL Error: {:?}", errors).into());
            }
            return response_body.data.ok_or_else(|| "No data".into());
        }

        let res = self
            .http_client
            .post(self.api_url)
            .json(&json_body)
            .send()
            .await?;

        let raw_response_text = res.text().await?;

        self.media_list_cache
            .insert(cache_key, raw_response_text.clone())
            .await;

        let response_body: graphql_client::Response<get_media::ResponseData> =
            serde_json::from_str(&raw_response_text)?;

        if let Some(errors) = response_body.errors {
            return Err(format!("GraphQL Error: {:?}", errors).into());
        }

        response_body.data.ok_or_else(|| "No data".into())
    }

    pub async fn get_media_details(
        &self,
        media_id: i64,
    ) -> Result<get_media_details::ResponseData, Box<dyn std::error::Error + Sync + Send>> {
        let variables = get_media_details::Variables {
            media_id,
            format: None,
        };

        let request_body = GetMediaDetails::build_query(variables);

        if let Some(cached_response) = self.details_cache.get(&media_id).await {
            let response_body: graphql_client::Response<get_media_details::ResponseData> =
                serde_json::from_str(&cached_response)?;

            if let Some(errors) = response_body.errors {
                return Err(format!("GraphQL Error: {:?}", errors).into());
            }
            return response_body.data.ok_or_else(|| "No data".into());
        }

        let res = self
            .http_client
            .post(self.api_url)
            .json(&request_body)
            .send()
            .await?;

        let raw_response_text = res.text().await?;

        self.details_cache
            .insert(media_id, raw_response_text.clone())
            .await;

        let response_body: graphql_client::Response<get_media_details::ResponseData> =
            serde_json::from_str(&raw_response_text)?;

        if let Some(errors) = response_body.errors {
            return Err(format!("GraphQL Error: {:?}", errors).into());
        }

        response_body.data.ok_or_else(|| "No data".into())
    }
    pub async fn update_entry(
        &self,
        user_media_details: &UserMediaDetails,
    ) -> Result<update_entry::ResponseData, Box<dyn std::error::Error + Sync + Send>> {
        let variables = update_entry::Variables {
            media_id: user_media_details.media_id,
            status: user_media_details.status,
            progress: user_media_details.progress,
            progress_volumes: user_media_details.progress_volumes,
            repeat: user_media_details.repeat,
            started_at: user_media_details.started_at.to_update_entry(),
            completed_at: user_media_details.completed_at.to_update_entry(),
            score: user_media_details.score,
            notes: Some(user_media_details.notes.clone()),
        };
        let request_body = UpdateEntry::build_query(variables);

        let json_body = serde_json::to_value(&request_body)?;
        let res = self
            .http_client
            .post(self.api_url)
            .json(&json_body)
            .send()
            .await?;

        let response_body: graphql_client::Response<update_entry::ResponseData> =
            res.json().await?;

        if let Some(errors) = response_body.errors {
            return Err(format!("GraphQL Error: {:?}", errors).into());
        }

        response_body.data.ok_or_else(|| "No data".into())
    }
    pub async fn toggle_favourite(
        &self,
        anime_id: Option<i64>,
        manga_id: Option<i64>,
    ) -> Result<toggle_favourite::ResponseData, Box<dyn std::error::Error + Sync + Send>> {
        let variables = toggle_favourite::Variables { anime_id, manga_id };

        let request_body = ToggleFavourite::build_query(variables);

        let res = self
            .http_client
            .post(self.api_url)
            .json(&request_body)
            .send()
            .await?;

        let response_body: graphql_client::Response<toggle_favourite::ResponseData> =
            res.json().await?;

        if let Some(errors) = response_body.errors {
            return Err(format!("GraphQL Error: {:?}", errors).into());
        }

        response_body.data.ok_or_else(|| "No data".into())
    }

    pub async fn delete_media(
        &self,
        media_id: i64,
    ) -> Result<toggle_favourite::ResponseData, Box<dyn std::error::Error + Sync + Send>> {
        let variables = delete_media_list_entry::Variables {
            delete_media_list_entry_id: media_id,
        };

        let request_body = DeleteMediaListEntry::build_query(variables);

        let res = self
            .http_client
            .post(self.api_url)
            .json(&request_body)
            .send()
            .await?;

        let response_body: graphql_client::Response<toggle_favourite::ResponseData> =
            res.json().await?;

        if let Some(errors) = response_body.errors {
            return Err(format!("GraphQL Error: {:?}", errors).into());
        }

        response_body.data.ok_or_else(|| "No data".into())
    }

    pub async fn update_details_cache_favourite(&self, media_id: i64, is_favourite: bool) {
        if let Some(cached_response) = self.details_cache.get(&media_id).await
            && let Ok(mut json_val) = serde_json::from_str::<serde_json::Value>(&cached_response)
            && let Some(data) = json_val.get_mut("data")
        {
            let media_opt = match data.get_mut("Media") {
                Some(m) => Some(m),
                None => data.get_mut("media"),
            };

            if let Some(media) = media_opt {
                if media.get("isFavourite").is_some() {
                    media["isFavourite"] = serde_json::Value::Bool(is_favourite);
                } else {
                    media["is_favourite"] = serde_json::Value::Bool(is_favourite);
                }

                if let Ok(updated_str) = serde_json::to_string(&json_val) {
                    self.details_cache.insert(media_id, updated_str).await;
                }
            }
        }
    }
    pub fn clear_media_list_cache(&self) {
        self.media_list_cache.invalidate_all();
    }
    pub async fn delete_from_details_cache(&self, media_id: i64) {
        self.details_cache.invalidate(&media_id).await;
    }
}
