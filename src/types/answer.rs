use serde::{Deserialize, Serialize};
use crate::types::question::{QuestionId, Question};
use crate::storage::store::Store;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct AnswerId(pub i32);

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Answer {
    pub id: AnswerId,
    pub content: String,
    pub question_id: QuestionId,
}
impl AnswerId {
    pub fn new(id: i32) -> Self {
        AnswerId(id)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NewAnswer {
    pub content: String,
    pub question_id: QuestionId,
}
