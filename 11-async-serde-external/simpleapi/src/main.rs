use std::{thread};
use std::time::Duration;
use articles::Articles;
use nix::unistd::gettid;
use rand::Rng;
use std::sync::{Arc, Mutex};
use tokio::runtime::Builder;
use serde_json::Value;
use serde::ser::{Serializer, SerializeStruct};
use serde::{Serialize};


extern crate  articles;

type ArticlesRef = Arc<Mutex<articles::Articles>>;

struct ArticleResponse<'res> { 
    article_ref: &'res articles::Article
}


fn main() {



    // Inicializace a plneni dat

    let article1: articles::Article = articles::Article { title: ("Cesta tam a zase zpatky".to_string()),
                                                            author: ("J.R.R Tolkiej".to_string()), 
                                                            description: ("Pohadkove vypraveni o hobitech".to_string()), 
                                                            content: ("Fantasy".to_string()) };

    let article2: articles::Article = articles::Article { title: ("Spolecentstvo prstenu".to_string()),
                                                            author: ("J.R.R Tolkiem".to_string()), 
                                                            description: ("Pokracovani hobita".to_string()), 
                                                            content: ("Fantasy".to_string()) };


    let mut articles = articles::Articles::new();
    articles.add(&article1);
    articles.add(&article2);


    // println!("Example whole collection in json {}", serde_json::to_string_pretty(&articles).unwrap());

    // Shared cast
    // https://doc.rust-lang.org/book/ch16-03-shared-state.html
    // Otazka - musi tam bejt ten mutex kdyz pouze cteme a nebo ne ?


    // let mutex_ref = Arc::new(Mutex::new(articles));
    let articles_r: ArticlesRef = Arc::new(Mutex::new(articles));
    
    let mut handles = vec![];

    let rt = Builder::new_multi_thread()
        .worker_threads(2)
        .thread_name("worker")
        .thread_stack_size(3 * 1024 * 1024)
        .build()
        .unwrap();

    rt.block_on(async {

        for _ in 1..10 {

            let articles_ref = Arc::clone(&articles_r);
            
            let handler = tokio::spawn(async move {
                task_handler(articles_ref).await;
            });

            handles.push(handler);
        }

        for handler in handles {
            handler.await.unwrap();
        }
    

    });


}

async fn task_handler (articles_ref: ArticlesRef) {

    let id = gettid();
    let mut rng = rand::thread_rng();
    let sleep_time = rng.gen_range(0..4);
    // let example: ArticleResponse;
    let example: &articles::Article;
    println!("I will sleep for {} sec in task number: {}", sleep_time, id);
    let articles_lock = articles_ref.lock();
    match articles_lock {
        Ok(ref a) => { 
            println!("Sucessuly locked in task with id {}", id);
            example = articles::Articles::get(&a,1).unwrap();
            thread::sleep(Duration::from_secs(sleep_time));
        }
        _ => {
            println!("Error when locking struct articles in task {}", id);
            return
        },
    
   };

    let article_ref: ArticleResponse = example.try_into().unwrap();
    // println!("Example has been filled in with values: {}", example);
    println!("Example in json {}", serde_json::to_string_pretty(&article_ref).unwrap());

    println!{"Thread cislo {} skoncil\n\n", id};

}

impl<'res> Serialize for ArticleResponse<'res> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("ArticleResponse", 4)?;
        s.serialize_field("title", &self.article_ref.title)?;
        s.serialize_field("author", &self.article_ref.author)?;
        s.serialize_field("description", &self.article_ref.description)?;
        s.serialize_field("content", &self.article_ref.content)?;
        s.end()
    }
}




impl<'res> TryFrom<&'res articles::Article> for ArticleResponse<'res> {

    type Error = ();

    fn try_from(f: &'res articles::Article) -> Result<Self, ()> {
        Ok(ArticleResponse { article_ref: f})
    }
}

