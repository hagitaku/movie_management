use sea_orm::prelude::Date;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub enum SortKey {
    #[serde(rename = "movieId")]
    MovieId,
    #[serde(rename = "title")]
    Title,
    #[serde(rename = "userName")]
    UserName,
    #[serde(rename = "date")]
    Date,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct MovieListRequest {
    pub movie_id: Option<i32>,
    pub title: Option<String>,
    pub created_date: Option<String>,
    pub description: Option<String>,
    pub user_id: Option<i32>,
    pub user_name: Option<String>,
    pub sort_key: Option<SortKey>,
    pub page: i32,
    pub count: i32,
}

impl MovieListRequest {
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.page < 1 {
            return Err("pageは1以上で指定してください");
        }
        if !(1..=100).contains(&self.count) {
            return Err("countは1〜100で指定してください");
        }
        if self.movie_id.is_some_and(|id| id < 1) || self.user_id.is_some_and(|id| id < 1) {
            return Err("movie_idとuser_idは1以上で指定してください");
        }
        if let Some(value) = &self.created_date {
            let valid = Date::parse_from_str(value, "%Y-%m-%d").is_ok_and(|date| {
                date.format("%Y-%m-%d").to_string() == *value && date.succ_opt().is_some()
            });
            if !valid {
                return Err("created_dateは実在する日付をYYYY-MM-DD形式で指定してください");
            }
        }
        Ok(())
    }
}

#[derive(Serialize, ToSchema)]
pub struct MovieListResponse {
    pub movie_list: Vec<MovieListItem>,
    pub total_count: i32,
    pub page: i32,
    pub count: i32,
}
#[derive(Serialize, ToSchema)]
pub struct MovieListItem {
    pub movie_id: i32,
    pub title: String,
    pub description: String,
    pub user_name: String,
    pub created_date: String,
    pub user_id: i32,
}
