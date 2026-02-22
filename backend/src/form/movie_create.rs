use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct MovieCreateRequest {
    pub title: String,
    pub description: String,
    pub memo: String,
    // 以下、TODO
    // pub genre: Genre, NOTE: Genre構造体はIDと名前を持ち、IDが外部キーとして使われる。IDが存在しない場合は新規登録される。
    // pub url: String, NOTE: URLは空文字列を許容する。
    // pub image: String, NOTE: 画像は空文字列を許容する。
}

impl MovieCreateRequest {
    fn valid_title(&self) -> bool {
        self.title.len() > 0 && self.title.len() <= 255
    }

    fn valid_description(&self) -> bool {
        self.description.len() <= 500
    }

    fn valid_memo(&self) -> bool {
        self.memo.len() <= 500
    }

    pub fn validation_movie_management(&self) -> String {
        if !self.valid_title() {
            return "タイトルは必須で255文字以内で入力してください".to_owned();
        }
        if !self.valid_description() {
            return "説明は500文字以内で入力してください".to_owned();
        }
        if !self.valid_memo() {
            return "メモは500文字以内で入力してください".to_owned();
        }
        "".to_owned()
    }
}

#[derive(Serialize, ToSchema)]
pub struct MovieCreateResponse {
    pub message: String,
}
