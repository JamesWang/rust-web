pub struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct QuestionId(String);

impl QuestionId {
    pub fn new(id: String) -> Self {
        QuestionId(id)
    }
}

impl Question {
    pub fn new(id: QuestionId, title: String, content: String, tags: Option<Vec<String>>) -> Self {
        Question {
            id,
            title,
            content,
            tags,
        }
    }
    pub fn update_title(&mut self, new_title: String) -> Self{
        Question::new(
            self.id.clone(),
            new_title,
            self.content.clone(),
            self.tags.clone(),
        )
    }

    pub fn id(&self) -> &str {
        &self.id.0
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn tags(&self) -> Option<&Vec<String>> {
        self.tags.as_ref()
    }
}

impl std::fmt::Debug for Question {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "Question ID: {}, Title: {}, Content: {}, Tags: {:?}",
            self.id.0, self.title, self.content, self.tags
        )
    }
}

impl std::fmt::Display for Question {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "Question ID: {}, Title: {}, Content: {}, Tags: {:?}",
            self.id.0, self.title, self.content, self.tags
        )
    }
}

/* impl std::fmt::Debug for QuestionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "id: {}", self.0)
    }
}
 */