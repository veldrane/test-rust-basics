

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Clone,Deserialize,Serialize)]
pub struct Article  {
    pub title: String,
    pub author: String,
    pub description: String,
    pub content: String
}

pub struct Articles {
    pub articles: Vec<Article>
}

pub enum ArticleErr {
    NotFound,
    Denied,
}

impl Articles {

    pub fn new() -> Self {
        Articles {articles: vec![]} 
    }

    pub fn add(&mut self, a: &Article) {
        self.articles.push(a.clone())
    }

    pub fn get(&self, id: usize) -> Option<&Article> {
        self.articles.get(id)
    }


    pub async fn get_json (&self) -> axum::extract::Json<Value> {
        match self.articles.get(1) {
            Some(a) => json!({"title": a.title, "description": a.description}).into(),
            None => unimplemented!()
        }
    }

}
