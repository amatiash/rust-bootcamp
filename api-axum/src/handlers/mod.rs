mod handlers_inner;

use crate::{AppState, models::*};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

impl IntoResponse for handlers_inner::HandlerError {
    fn into_response(self) -> axum::response::Response {
        match self {
            handlers_inner::HandlerError::BadRequest(msg) => {
                (StatusCode::BAD_REQUEST, msg).into_response()
            }
            handlers_inner::HandlerError::InternalError(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response()
            }
        }
    }
}

// ---- CRUD for Questions ----

pub async fn create_question(
    State(AppState { questions_dao, .. }): State<AppState>,
    Json(question): Json<Question>,
) -> impl IntoResponse {
    // TODO: update return type to be of type `Result`. Both the Ok and Err case should contain `impl IntoResponse`.
    // TODO: Replace the fake data below with a call to `handlers_inner::create_question`.
    // Return the result wrapped in JSON in the success case and an `HandlerError` in the error case.
    // NOTE: `IntoResponse` is implemented for `HandlerError` above.
    let question = QuestionDetail {
        question_uuid: "d347261c-3f0e-42d2-8706-5ef9f1b96725".to_string(),
        title: question.title,
        description: question.description,
        created_at: "2022-12-31 18:44:08.287442".to_string(),
    };

    Json(question)
}

pub async fn read_questions(
    State(AppState { questions_dao, .. }): State<AppState>,
) -> impl IntoResponse {
    // TODO: update return type to be of type `Result`. Both the Ok and Err case should contain `impl IntoResponse`.
    // TODO: Replace the fake data below with a call to `handlers_inner::read_questions`.
    // Return the result wrapped in JSON in the success case and an `HandlerError` in the error case.
    // NOTE: `IntoResponse` is implemented for `HandlerError` above.
    let question = QuestionDetail {
        question_uuid: "d347261c-3f0e-42d2-8706-5ef9f1b96725".to_string(),
        title: "Newly Created Question".to_string(),
        description: "My Description".to_string(),
        created_at: "2022-12-31 18:44:08.287442".to_string(),
    };

    Json(vec![question])
}

pub async fn delete_question(
    State(AppState { questions_dao, .. }): State<AppState>,
    Json(question_uuid): Json<QuestionId>,
) {
    // TODO: update return type to be of type `Result`. Both the Ok and Err case should contain `impl IntoResponse`.
    // TODO: Make a call to `handlers_inner::delete_question`.
    // Return a unit type in the success case and an `HandlerError` in the error case.
    // NOTE: `IntoResponse` is implemented for `HandlerError` above.
}

// ---- CRUD for Answers ----

pub async fn create_answer(
    State(AppState { answers_dao, .. }): State<AppState>,
    Json(answer): Json<Answer>,
) -> impl IntoResponse {
    // TODO: update return type to be of type `Result`. Both the Ok and Err case should contain `impl IntoResponse`.
    // TODO: Replace the fake data below with a call to `handlers_inner::create_answer`.
    // Return the result wrapped in JSON in the success case and an `HandlerError` in the error case.
    // NOTE: `IntoResponse` is implemented for `HandlerError` above.
    let answer = AnswerDetail {
        answer_uuid: "a1a14a9c-ab9e-481b-8120-67f675531ed2".to_string(),
        question_uuid: answer.question_uuid,
        content: answer.content,
        created_at: "2022-12-31 13:11:59.728682".to_string(),
    };

    Json(answer)
}

pub async fn read_answers(
    State(AppState { answers_dao, .. }): State<AppState>,
    Json(question_uuid): Json<QuestionId>,
) -> impl IntoResponse {
    // TODO: update return type to be of type `Result`. Both the Ok and Err case should contain `impl IntoResponse`.
    // TODO: Replace the fake data below with a call to `handlers_inner::read_answers`.
    // Return the result wrapped in JSON in the success case and an `HandlerError` in the error case.
    // NOTE: `IntoResponse` is implemented for `HandlerError` above.
    let answer = AnswerDetail {
        answer_uuid: "a1a14a9c-ab9e-481b-8120-67f675531ed2".to_string(),
        question_uuid: "b068cd2f-edac-479e-98f1-c5f91008dcbd".to_string(),
        content: "test question".to_string(),
        created_at: "2022-12-31 13:11:59.728682".to_string(),
    };

    Json(vec![answer])
}

pub async fn delete_answer(
    State(AppState { answers_dao, .. }): State<AppState>,
    Json(answer_uuid): Json<AnswerId>,
) {
    // TODO: update return type to be of type `Result`. Both the Ok and Err case should contain `impl IntoResponse`.
    // TODO: Make a call to `handlers_inner::delete_answer`.
    // Return a unit type in the success case and an `HandlerError` in the error case.
    // NOTE: `IntoResponse` is implemented for `HandlerError` above.
}
