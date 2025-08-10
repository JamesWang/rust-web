use core::error;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde_json;
use crate::types::{
    answer::{Answer, AnswerId, NewAnswer},
    question::{self, NewQuestion, Question, QuestionId},
};
use sqlx::postgres::{PgPoolOptions, PgPool, PgRow};
use sqlx::Row;
use handle_errors::Error;
use crate::types::account::{Account, NewAccount};

#[derive(Debug, Clone)]
pub struct Store {
    //pub questions: Arc<RwLock<HashMap<QuestionId, Question>>>,
    //pub answers: Arc<RwLock<HashMap<AnswerId, Answer>>>,
    pub connection: PgPool,
}


impl Store {
    pub async fn new(db_url: &str) -> Self {
        let db_pool = match PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await
        {
            Ok(pool) => pool,
            Err(e) => panic!("Failed to create database connection pool: {}", e),
        };

        Store {
            connection: db_pool,
        }
    }

/*     fn init() -> HashMap<QuestionId, Question> {
        let file = include_str!("../../questions.json");

        //the following returns a HashMap<QuestionId, Question> loaded from the JSON file using serde_json
        serde_json::from_str(file).expect("Cannot read questions.json")
    }
 */
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

   pub async fn get_questions(&self, limit: Option<u32>, offset: u32) -> Result<Vec<Question>, Error> {
        match sqlx::query("SELECT id, title, content, tags FROM questions LIMIT $1 OFFSET $2")
            .bind(limit.map(|v| v as i32)) // Default limit if None
            .bind(offset as i64)
            .map(|row: PgRow| Question::new(
                    QuestionId::new(row.get::<i32, _>("id")),
                    row.get("title"),
                    row.get("content"),
                    row.get::<Option<Vec<String>>, _>("tags"),
                ))
                .fetch_all(&self.connection)
                .await {
                    Ok(questions) => Ok(questions),
                    Err(e) => {
                        tracing::event!(tracing::Level::ERROR, "Error fetching questions: {:?}", e);
                        Err(Error::DatabaseQueryError(e))
            }
        }
    }

    pub async fn add_question(&self, new_question: NewQuestion) -> Result<Question, sqlx::Error> {
        match sqlx::query("INSERT INTO questions (title, content, tags) VALUES ($1, $2, $3) RETURNING id, title, content, tags")
            .bind(new_question.title)
            .bind(new_question.content)
            .bind(new_question.tags)
            .map(|row:PgRow| {
                Question::new(
                    QuestionId::new(row.get::<i32, _>("id")),
                    row.get("title"),
                    row.get("content"),
                    row.get::<Option<Vec<String>>, _>("tags"),
                )
            })
            .fetch_one(&self.connection)            
            .await
            {
                Ok(q) => Ok(q),
                Err(e) => Err(e)
            }        
    }

    pub async fn update_question(&self, id: QuestionId, question: Question) -> Result<Question, sqlx::Error> {
        match sqlx::query("UPDATE questions SET title = $1, content = $2, tags = $3 WHERE id = $4 RETURNING id, title, content, tags")
            .bind(question.title())
            .bind(question.content())
            .bind(question.tags())
            .bind(id.0)
            .map(|row: PgRow| {
                Question::new(
                    QuestionId::new(row.get::<i32, _>("id")),
                    row.get("title"),
                    row.get("content"),
                    row.get::<Option<Vec<String>>, _>("tags"),
                )
            })
            .fetch_one(&self.connection)            
            .await {
                Ok(res) => Ok(res),
                Err(e) => Err(e)
            }        
    }

    pub async fn delete_question(&self, id: QuestionId) -> Result<bool, sqlx::Error> {
        match sqlx::query("DELETE FROM questions WHERE id = $1")
            .bind(id.0)
            .execute(&self.connection)
            .await {
                Ok(_) => Ok(true),
                Err(e) => Err(e)
            }
    }
    
    pub async fn add_answer(
        &self,        
        new_answer: NewAnswer,
    ) -> Result<Answer, sqlx::Error> {
        match sqlx::query("INSERT INTO answers (content, question_id) VALUES ($1, $2) RETURNING id, content, question_id")
            .bind(new_answer.content)
            .bind(new_answer.question_id.0)
            .map(|row: PgRow| {
                Answer {
                    id: AnswerId::new(row.get::<i32, _>("id")),
                    content: row.get("content"),
                    question_id: QuestionId::new(row.get::<i32, _>("question_id")),
                }
            })
            .fetch_one(&self.connection)
            .await {
                Ok(answer) => Ok(answer),
                Err(e) => Err(e),
            }
    }

    pub async fn add_account(&self, account: Account) -> Result<bool, Error> {
        match sqlx::query("INSERT INTO accounts (email, password) VALUES ($1, $2)")
            .bind(account.email)
            .bind(account.password)
            .execute(&self.connection)
            .await {
                Ok(_) => Ok(true),
                Err(error) => {
                    tracing::event!(
                        tracing::Level::ERROR, 
                        code = error
                                .as_database_error()
                                .unwrap()
                                .code()
                                .unwrap()
                                .parse::<i32>()
                                .unwrap(),
                        db_message = error
                                .as_database_error()
                                .unwrap()
                                .message(),
                         constraint = error
                                .as_database_error()
                                .unwrap()
                                .constraint()
                                .unwrap()
                    );
                    Err(Error::DatabaseQueryError(error))
                }
            }
    }
}