use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde_json;
use crate::types::answer::{Answer, AnswerId};
use crate::types::question::{Question, QuestionId};

#[derive(Debug, Clone)]
pub struct Store {
    pub questions: Arc<RwLock<HashMap<QuestionId, Question>>>,
    pub answers: Arc<RwLock<HashMap<AnswerId, Answer>>>,
}


impl Store {
    pub fn new() -> Self {
        Store {
            questions: Arc::new(RwLock::new(Self::init())),
            answers: Arc::new(RwLock::new(HashMap::new())), // Initialize with an empty HashMap for answers
        }
    }

    fn init() -> HashMap<QuestionId, Question> {
        let file = include_str!("../../questions.json");

        //the following returns a HashMap<QuestionId, Question> loaded from the JSON file using serde_json
        serde_json::from_str(file).expect("Cannot read questions.json")
    }

/*     pub fn add_question(&mut self, question: Question) -> Self{
        self.questions.insert(question.id().clone(), question);
        self
    } */

/*     pub fn get_question(&self, id: &QuestionId) -> Option<&Question> {
        self.questions.get(id)
    }

    pub fn get_all_questions(&self) -> Vec<&Question> {
        self.questions.values().collect()
    } */
}