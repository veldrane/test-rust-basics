use std::fmt::{Formatter, Error, Display};
use serde::{Serialize, Deserialize};

#[derive(Clone,Serialize,Deserialize)]
pub struct Article  {
    pub title: String,
    pub author: String,
    pub description: String,
    pub content: String
}

#[derive(Serialize,Deserialize)]
pub struct Articles {
    pub articles: Vec<Article>
}

pub enum ArticleErr {
    NotFound,
    Denied,
}

// Protoze pocitame s tim ze to options vrati vzdy author pak ten zapis muze vypadata takto


impl Articles {

    pub fn new() -> Self {
        Articles {articles: vec![]} 
    }

    pub fn add(&mut self, a: &Article) {
        self.articles.push(a.clone())
    }

    pub fn get(&self, id: usize) -> Option<&Article> {

        match self.articles.get(id) {
            Some(a) => return Some(a),
            None => None
        }
    }

    pub fn get_clone(&self, id: usize) -> Option<Article> {

        match self.articles.get(id).cloned() {
            Some(a) => return Some(a),
            None => None
        }
    }

}

impl Article {
    pub fn new(author: String, title: String, content: String, description: String) -> Self {

        Self {  title: (title.to_string()), 
                author: (author.to_string()), 
                description: (description.to_string()), 
                content: (content.to_string()) 
            }
    }
}

impl Display for Article {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
            write!(f, "\n --- \n Title: {}\n Author:{}\n Description:{}\n Content:{}\n --- \n", self.title, self.author, self.description, self.content)
    }
}