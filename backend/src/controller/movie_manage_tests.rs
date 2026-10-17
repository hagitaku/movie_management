use super::movie_search;
use crate::controller::orm::movie_manage::search_movies;
use crate::form::movie_list::{MovieListRequest, SortKey};
use crate::model::{account, movie};
use crate::setting::AppState;
use actix_web::{http::StatusCode, test, web, App};
use sea_orm::{
    ActiveModelTrait, ConnectionTrait, Database, DatabaseConnection, EntityTrait, Schema, Set,
};
use serde_json::{json, Value};

fn request(value: Value) -> MovieListRequest {
    serde_json::from_value(value).unwrap()
}

async fn database() -> DatabaseConnection {
    let conn = Database::connect("sqlite::memory:").await.unwrap();
    let backend = conn.get_database_backend();
    let schema = Schema::new(backend);
    conn.execute(backend.build(&schema.create_table_from_entity(account::Entity)))
        .await
        .unwrap();
    conn.execute(backend.build(&schema.create_table_from_entity(movie::Entity)))
        .await
        .unwrap();

    for (id, name) in [(1, "alice"), (2, "bob")] {
        account::ActiveModel {
            id: Set(id),
            login_id: Set(name.to_owned()),
            password: Set("private".to_owned()),
        }
        .insert(&conn)
        .await
        .unwrap();
    }
    for (id, title, user, date, description) in [
        (1, "Zulu", 1, "2026-10-12 00:00:00", "宇宙の映画"),
        (2, "Alpha", 2, "2026-10-11 23:59:59", "冒険の映画"),
        (3, "Alpha", 1, "2026-10-12 23:59:59", "宇宙と冒険"),
        (4, "100%_!", 2, "2026-10-13 00:00:00", "記号を含む映画"),
    ] {
        movie::ActiveModel {
            id: Set(id),
            title: Set(title.to_owned()),
            created_user_id: Set(user),
            description: Set(description.to_owned()),
            memo: Set("memo".to_owned()),
            created_at: Set(
                sea_orm::prelude::DateTime::parse_from_str(date, "%Y-%m-%d %H:%M:%S").unwrap(),
            ),
        }
        .insert(&conn)
        .await
        .unwrap();
    }
    conn
}

#[actix_web::test]
async fn pagination_returns_filtered_total_and_empty_out_of_range_page() {
    let conn = database().await;
    let response = search_movies(&conn, &request(json!({"page": 2, "count": 2})))
        .await
        .unwrap();
    assert_eq!(response.total_count, 4);
    assert_eq!(
        response
            .movie_list
            .iter()
            .map(|item| item.movie_id)
            .collect::<Vec<_>>(),
        vec![3, 4]
    );
    assert_eq!((response.page, response.count), (2, 2));

    let response = search_movies(&conn, &request(json!({"page": 3, "count": 2})))
        .await
        .unwrap();
    assert!(response.movie_list.is_empty());
    assert_eq!(response.total_count, 4);

    let response = search_movies(
        &conn,
        &request(json!({"page": 1, "count": 2, "title": "missing"})),
    )
    .await
    .unwrap();
    assert!(response.movie_list.is_empty());
    assert_eq!(response.total_count, 0);
}

#[actix_web::test]
async fn combines_all_filters_and_includes_the_whole_creation_day() {
    let conn = database().await;
    let response = search_movies(
        &conn,
        &request(json!({
            "page": 1, "count": 1, "created_date": "2026-10-12", "description": "宇宙"
        })),
    )
    .await
    .unwrap();
    assert_eq!(response.total_count, 2);
    assert_eq!(response.movie_list[0].movie_id, 1);
    let response = search_movies(
        &conn,
        &request(json!({
            "page": 2, "count": 1, "created_date": "2026-10-12"
        })),
    )
    .await
    .unwrap();
    assert_eq!(response.total_count, 2);
    assert_eq!(response.movie_list[0].movie_id, 3);

    let response = search_movies(
        &conn,
        &request(json!({
            "page": 1, "count": 10, "movie_id": 3, "title": "lph", "user_id": 1,
            "user_name": "lic", "description": "冒険", "created_date": "2026-10-12"
        })),
    )
    .await
    .unwrap();
    assert_eq!(response.total_count, 1);
    assert_eq!(response.movie_list[0].movie_id, 3);
    assert_eq!(response.movie_list[0].created_date, "2026-10-12");

    let response = search_movies(
        &conn,
        &request(json!({
            "page": 1, "count": 10, "movie_id": 3, "user_id": 2
        })),
    )
    .await
    .unwrap();
    assert_eq!(response.total_count, 0);
}

#[actix_web::test]
async fn sorting_has_deterministic_ties() {
    let conn = database().await;
    for (key, expected) in [
        (SortKey::MovieId, vec![1, 2, 3, 4]),
        (SortKey::Title, vec![4, 2, 3, 1]),
        (SortKey::UserName, vec![1, 3, 2, 4]),
        (SortKey::Date, vec![2, 1, 3, 4]),
    ] {
        let mut req = request(json!({"page": 1, "count": 10}));
        req.sort_key = Some(key);
        let response = search_movies(&conn, &req).await.unwrap();
        assert_eq!(
            response
                .movie_list
                .iter()
                .map(|item| item.movie_id)
                .collect::<Vec<_>>(),
            expected
        );
    }
    let response = search_movies(
        &conn,
        &request(json!({
            "page": 2, "count": 1, "sort_key": "title", "title": "Alpha"
        })),
    )
    .await
    .unwrap();
    assert_eq!(response.movie_list[0].movie_id, 3);
    assert_eq!(response.total_count, 2);
}

#[actix_web::test]
async fn wildcard_characters_are_literal_search_text() {
    let conn = database().await;
    for title in ["%", "_", "!", "100%_!"] {
        let response = search_movies(
            &conn,
            &request(json!({
                "page": 1, "count": 10, "title": title
            })),
        )
        .await
        .unwrap();
        assert_eq!(response.total_count, 1);
        assert_eq!(response.movie_list[0].movie_id, 4);
    }
}

#[actix_web::test]
async fn endpoint_reflects_account_rename_and_excludes_private_fields() {
    let conn = database().await;
    let account = account::Entity::find_by_id(1u32)
        .one(&conn)
        .await
        .unwrap()
        .unwrap();
    let mut account: account::ActiveModel = account.into();
    account.login_id = Set("renamed".to_owned());
    account.update(&conn).await.unwrap();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState { conn }))
            .service(movie_search),
    )
    .await;
    let response = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/movie/search")
            .set_json(json!({"page": 1, "count": 10, "user_name": "renamed"}))
            .to_request(),
    )
    .await;
    assert_eq!(response.status(), StatusCode::OK);
    let body: Value = test::read_body_json(response).await;
    assert_eq!(body["total_count"], 2);
    assert_eq!(body["movie_list"][0]["user_name"], "renamed");
    assert_eq!(body["movie_list"][0]["user_id"], 1);
    assert!(body["movie_list"][0].get("password").is_none());
    assert!(body["movie_list"][0].get("memo").is_none());
}

#[actix_web::test]
async fn endpoint_rejects_invalid_input_before_accessing_database() {
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState {
                conn: DatabaseConnection::Disconnected,
            }))
            .service(movie_search),
    )
    .await;
    for body in [
        json!({"page": 0, "count": 10}),
        json!({"page": -1, "count": 10}),
        json!({"page": 1, "count": 0}),
        json!({"page": 1, "count": -1}),
        json!({"page": 1, "count": 101}),
        json!({"page": 1, "count": 10, "movie_id": 0}),
        json!({"page": 1, "count": 10, "user_id": -1}),
        json!({"page": 1, "count": 10, "created_date": "2026-02-30"}),
        json!({"page": 1, "count": 10, "created_date": "2026-1-1"}),
        json!({"page": 1, "count": 10, "sort_key": "invalid"}),
        json!({"count": 10}),
    ] {
        let response = test::call_service(
            &app,
            test::TestRequest::post()
                .uri("/movie/search")
                .set_json(body)
                .to_request(),
        )
        .await;
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}

#[actix_web::test]
async fn endpoint_returns_generic_error_when_database_query_fails() {
    let conn = Database::connect("sqlite::memory:").await.unwrap();
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState { conn }))
            .service(movie_search),
    )
    .await;
    let response = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/movie/search")
            .set_json(json!({"page": 1, "count": 10}))
            .to_request(),
    )
    .await;
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    let body: Value = test::read_body_json(response).await;
    assert_eq!(body["message"], "映画一覧の取得に失敗しました");
}
