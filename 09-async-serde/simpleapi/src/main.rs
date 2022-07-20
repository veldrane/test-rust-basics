use std::{thread};
use std::time::Duration;
use nix::unistd::gettid;
use rand::Rng;
use std::sync::{Arc, Mutex};
use tokio::task;


extern crate  articles;

type ArticlesRef = Arc<Mutex<articles::Articles>>;

#[tokio::main]
async fn main() {



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

    // Shared cast
    // https://doc.rust-lang.org/book/ch16-03-shared-state.html
    // Otazka - musi tam bejt ten mutex kdyz pouze cteme a nebo ne ?


    // let mutex_ref = Arc::new(Mutex::new(articles));
    let articles_r = Arc::new(Mutex::new(articles));
    
    let mut handles = vec![];

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

}

async fn task_handler (articles_ref: ArticlesRef) {

    let id = gettid();
    let mut rng = rand::thread_rng();
    let sleep_time = rng.gen_range(0..10);
    println!("I will sleep for {} sec in task number: {}", sleep_time, id);
    match articles_ref.lock() {
        Ok(a) => { 
            println!("Sucessuly locked in task with id {}", id);
            println!("pub articles vraci: {}", articles::Articles::get(&a,1).unwrap());
            thread::sleep(Duration::from_secs(sleep_time));
        }
        _ => println!("Error when locking struct articles in task {}", id),
   };

    println!{"Thread cislo {} skoncil\n\n", id};
}