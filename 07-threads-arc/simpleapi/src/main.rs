use std::{thread};
use std::time::Duration;
use rand::Rng;
use std::sync::{Arc, Mutex};

extern crate  articles;

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

    // Shared cast
    // https://doc.rust-lang.org/book/ch16-03-shared-state.html
    // Otazka - musi tam bejt ten mutex kdyz pouze cteme a nebo ne ?


    // let mutex_ref = Arc::new(Mutex::new(articles));
    let articles_r = Arc::new(articles);
    
    let mut handles = vec![];

    for i in 1..10 {

        let articles_ref = Arc::clone(&articles_r);

        let handler = thread::spawn(move || {
            let mut rng = rand::thread_rng();
            let sleep_time = rng.gen_range(0..10);
            println!("I will sleep for {} sec from thread number: {}", sleep_time, i);
            println!("pub articles vraci: {}", articles::Articles::get(&articles_ref,1).unwrap());
            thread::sleep(Duration::from_secs(sleep_time));
           //  match articles_ref.lock() {
           //     Ok(a) => { 
           //         println!("Sucessuly locked in thread {}", i);
           //         println!("pub articles vraci: {}", articles::Articles::get(&a,1).unwrap());
           //         thread::sleep(Duration::from_secs(sleep_time));

           //     }
           //     _ => println!("Error when locking struct articles in thread {}", i),
           // };

            println!{"Thread cislo {} skoncil\n\n", i}
        });

        handles.push(handler);
    }

    for handler in handles {
        handler.join().unwrap();
    }

}

