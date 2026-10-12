use crate::form::movie_create::MovieCreateRequest;

/**
 * 映画登録のバリデーション
 */
pub fn validation_movie_management(request: &MovieCreateRequest) -> String {
    request.validation_movie_management()
}
