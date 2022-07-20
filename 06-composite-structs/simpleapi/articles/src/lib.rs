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

// Protoze pocitame s tim ze to options vrati vzdy author pak ten zapis muze vypadata takto

pub fn get_author (articles: &Articles, id: usize) -> String {
    Articles::get(articles, id).unwrap().author
}

pub fn get_author_ref (articles: &Articles, id: usize) -> &String {
    &Articles::get_reference(articles, id).unwrap().author
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

    pub fn get_reference(&self, id: usize) -> Option<&Article> { // stejna fce - jednou vraci options a jednou Result a pokazdy je to reference ne clone

        self.articles.get(id)
    }

    pub fn get_reference_error(&self, id: usize) -> Result<&Article, ArticleErr> {

        match self.articles.get(id) {
            Some(a) => Ok(a),
            None => Err(ArticleErr::NotFound)
        }
    }

}
