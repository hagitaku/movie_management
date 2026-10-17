use crate::form::movie_create::MovieCreateRequest;
use crate::form::movie_list::{MovieListItem, MovieListRequest, MovieListResponse, SortKey};
use crate::model::movie::{ActiveModel, Model};
use crate::model::{account, movie};
use actix_web::web;
use chrono::Local;
use sea_orm::prelude::{Date, DateTime};
use sea_orm::sea_query::{LikeExpr, SimpleExpr};
use sea_orm::{
    entity::ActiveValue, ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait,
    FromQueryResult, JoinType, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, RelationTrait,
    Select,
};

pub async fn insert_movie(
    conn: &DatabaseConnection,
    request: &web::Json<MovieCreateRequest>,
    account_id: u32,
) -> Result<Model, sea_orm::DbErr> {
    let data = ActiveModel {
        id: ActiveValue::NotSet,
        title: ActiveValue::Set(request.title.clone()),
        description: ActiveValue::Set(request.description.clone()),
        memo: ActiveValue::Set(request.memo.clone()),
        created_user_id: ActiveValue::Set(account_id),
        created_at: ActiveValue::Set(Local::now().naive_local()),
    };

    data.insert(conn).await
}

#[derive(FromQueryResult)]
struct MovieListRow {
    movie_id: u32,
    title: String,
    description: String,
    user_name: String,
    created_at: DateTime,
    user_id: u32,
}

fn contains_literal(column: impl ColumnTrait, value: &str) -> SimpleExpr {
    let value = value
        .replace('!', "!!")
        .replace('%', "!%")
        .replace('_', "!_");
    column
        .into_expr()
        .like(LikeExpr::new(format!("%{value}%")).escape('!'))
}

fn search_query(request: &MovieListRequest) -> Result<Select<movie::Entity>, DbErr> {
    let mut query = movie::Entity::find().join(JoinType::InnerJoin, movie::Relation::Account.def());

    if let Some(id) = request.movie_id {
        query = query.filter(movie::Column::Id.eq(id));
    }
    if let Some(title) = &request.title {
        query = query.filter(contains_literal(movie::Column::Title, title));
    }
    if let Some(description) = &request.description {
        query = query.filter(contains_literal(movie::Column::Description, description));
    }
    if let Some(id) = request.user_id {
        query = query.filter(movie::Column::CreatedUserId.eq(id));
    }
    if let Some(name) = &request.user_name {
        query = query.filter(contains_literal(account::Column::LoginId, name));
    }
    if let Some(date) = &request.created_date {
        let date = Date::parse_from_str(date, "%Y-%m-%d")
            .map_err(|_| DbErr::Custom("Invalid created_date".to_owned()))?;
        let start = date
            .and_hms_opt(0, 0, 0)
            .ok_or_else(|| DbErr::Custom("Invalid created_date".to_owned()))?;
        let end = date
            .succ_opt()
            .and_then(|day| day.and_hms_opt(0, 0, 0))
            .ok_or_else(|| DbErr::Custom("Invalid created_date".to_owned()))?;
        query = query
            .filter(movie::Column::CreatedAt.gte(start))
            .filter(movie::Column::CreatedAt.lt(end));
    }

    Ok(query)
}

pub async fn search_movies(
    conn: &DatabaseConnection,
    request: &MovieListRequest,
) -> Result<MovieListResponse, DbErr> {
    request
        .validate()
        .map_err(|message| DbErr::Custom(message.to_owned()))?;
    let mut query = search_query(request)?;
    let total_count = query.clone().count(conn).await?;

    query = match request.sort_key.as_ref().unwrap_or(&SortKey::MovieId) {
        SortKey::MovieId => query,
        SortKey::Title => query.order_by_asc(movie::Column::Title),
        SortKey::UserName => query.order_by_asc(account::Column::LoginId),
        SortKey::Date => query.order_by_asc(movie::Column::CreatedAt),
    };
    // 同じソート値の映画もID順に固定して、ページングの順序を安定させる。
    let rows = query
        .order_by_asc(movie::Column::Id)
        .select_only()
        .column_as(movie::Column::Id, "movie_id")
        .column(movie::Column::Title)
        .column(movie::Column::Description)
        .column_as(account::Column::LoginId, "user_name")
        .column(movie::Column::CreatedAt)
        .column_as(movie::Column::CreatedUserId, "user_id")
        .offset((request.page as u64 - 1) * request.count as u64)
        .limit(request.count as u64)
        .into_model::<MovieListRow>()
        .all(conn)
        .await?;

    let movie_list = rows
        .into_iter()
        .map(|row| {
            Ok(MovieListItem {
                movie_id: i32::try_from(row.movie_id)
                    .map_err(|_| DbErr::Custom("Movie ID exceeds response range".to_owned()))?,
                title: row.title,
                description: row.description,
                user_name: row.user_name,
                created_date: row.created_at.format("%Y-%m-%d").to_string(),
                user_id: i32::try_from(row.user_id)
                    .map_err(|_| DbErr::Custom("User ID exceeds response range".to_owned()))?,
            })
        })
        .collect::<Result<Vec<_>, DbErr>>()?;

    Ok(MovieListResponse {
        movie_list,
        total_count: i32::try_from(total_count)
            .map_err(|_| DbErr::Custom("Movie count exceeds response range".to_owned()))?,
        page: request.page,
        count: request.count,
    })
}
