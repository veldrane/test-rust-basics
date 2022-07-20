#[derive(Clone)]
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

    pub fn get(&self, id: usize) -> Option<Article> {

        match self.articles.get(id) {
            Some(a) => return Some(a.clone()),
            None => None
        }
    }

    pub fn get_reference(&self, id: usize) -> Option<&Article> {

        self.articles.get(id)
    }

    pub fn get_reference_error(&self, id: usize) -> Result<&Article, ArticleErr> {

        match self.articles.get(id) {
            Some(a) => Ok(a),
            None => Err(ArticleErr::NotFound)
        }
    }

}
