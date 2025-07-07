mod handlers_inner;

use crate::{models::*, AppState};
use axum::{Json, response::IntoResponse, extract::State};

// ---- CRUD for Questions ----

pub async fn create_question(
    // Example of how to add state to a route. Note that we are using ".." to ignore the other fields in AppState.
    State(AppState { questions_dao, .. }): State<AppState>,
    Json(question): Json<Question>,
) -> impl IntoResponse {
    let question = QuestionDetail {
        question_uuid: "d347261c-3f0e-42d2-8706-5ef9f1b96725".to_string(),
        title: question.title,
        description: question.description,
        created_at: "2022-12-31 18:44:08.287442".to_string(),
    };

    Json(question)
}

pub async fn read_questions(// TODO: add questions_dao from app state as an argument
) -> impl IntoResponse {
    let question = QuestionDetail {
        question_uuid: "d347261c-3f0e-42d2-8706-5ef9f1b96725".to_string(),
        title: "Newly Created Question".to_string(),
        description: "My Description".to_string(),
        created_at: "2022-12-31 18:44:08.287442".to_string(),
    };

    Json(vec![question])
}

pub async fn delete_question(
    // TODO: add questions_dao from app state as an argument
    Json(question_uuid): Json<QuestionId>,
) {
    // ...
}

// ---- CRUD for Answers ----

pub async fn create_answer(
    // Example of how to add state to a route
    State(AppState { answers_dao, .. }): State<AppState>,
    Json(answer): Json<Answer>,
) -> impl IntoResponse {
    let answer = AnswerDetail {
        answer_uuid: "a1a14a9c-ab9e-481b-8120-67f675531ed2".to_string(),
        question_uuid: answer.question_uuid,
        content: answer.content,
        created_at: "2022-12-31 13:11:59.728682".to_string(),
    };

    Json(answer)
}

pub async fn read_answers(
    // TODO: add answers_dao from app state as an argument
    Json(question_uuid): Json<QuestionId>,
) -> impl IntoResponse {
    let answer = AnswerDetail {
        answer_uuid: "a1a14a9c-ab9e-481b-8120-67f675531ed2".to_string(),
        question_uuid: "b068cd2f-edac-479e-98f1-c5f91008dcbd".to_string(),
        content: "test question".to_string(),
        created_at: "2022-12-31 13:11:59.728682".to_string(),
    };

    Json(vec![answer])
}

pub async fn delete_answer(
    // TODO: add answers_dao from app state as an argument
    Json(answer_uuid): Json<AnswerId>,
) {
    // ...
}
