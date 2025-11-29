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
