mod handlers_inner;

use crate::models::*;
use axum::{Json, response::IntoResponse};

// ---- CRUD for Questions ----

pub async fn create_question(Json(question): Json<Question>) -> impl IntoResponse {
    let question = QuestionDetail {
        question_uuid: "d347261c-3f0e-42d2-8706-5ef9f1b96725".to_string(),
        title: question.title,
        description: question.description,
        created_at: "2022-12-31 18:44:08.287442".to_string(),
    };

    Json(question)
}

pub async fn read_questions() -> impl IntoResponse {
    let question = QuestionDetail {
        question_uuid: "d347261c-3f0e-42d2-8706-5ef9f1b96725".to_string(),
        title: "Newly Created Question".to_string(),
        description: "My Description".to_string(),
        created_at: "2022-12-31 18:44:08.287442".to_string(),
    };

    Json(vec![question])
}

pub async fn delete_question(Json(_question_uuid): Json<QuestionId>) {
    // To be implemented
}

// ---- CRUD for Answers ----

pub async fn create_answer(Json(answer): Json<Answer>) -> impl IntoResponse {
    let answer = AnswerDetail {
        answer_uuid: "a1a14a9c-ab9e-481b-8120-67f675531ed2".to_string(),
        question_uuid: answer.question_uuid,
        content: answer.content,
        created_at: "2022-12-31 13:11:59.728682".to_string(),
    };

    Json(answer)
}

pub async fn read_answers(Json(_question_uuid): Json<QuestionId>) -> impl IntoResponse {
    let answer = AnswerDetail {
        answer_uuid: "a1a14a9c-ab9e-481b-8120-67f675531ed2".to_string(),
        question_uuid: "b068cd2f-edac-479e-98f1-c5f91008dcbd".to_string(),
        content: "test question".to_string(),
        created_at: "2022-12-31 13:11:59.728682".to_string(),
    };

    Json(vec![answer])
}

pub async fn delete_answer(Json(_answer_uuid): Json<AnswerId>) {
    // To be implemented
}
