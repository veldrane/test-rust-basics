use articles::{ArticleErr, get_author, get_author_ref};

extern crate  articles;

fn main() {

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


    match articles.get_reference(0) {
        Some(a) => println!("Funkce vraci article {}", a.title),
        None => println!("Kniha nebyla nalezena")
    }
    
    match articles.get_reference(2) {
        Some(a) => println!("Funkce vraci article {}", a.title),
        None => println!("Kniha nebyla nalezena")
    }

    match articles.get_reference_error(0) {
        Ok(a) => println!("Funkce vraci article {}", a.title),
        Err(ArticleErr::NotFound) => println!("Kniha nebyla nalezena"),
        Err(ArticleErr::Denied) => println!("Pristup odeprecen")
    }
    
    match articles.get_reference_error(2) {
        Ok(a) => println!("Funkce vraci article {}", a.title),
        Err(ArticleErr::NotFound) => println!("Kniha nebyla nalezena"),
        Err(ArticleErr::Denied) => unimplemented!()
    }

    let boxed = Box::new(get_author); // https://stackoverflow.com/questions/35201250/what-is-the-difference-between-using-the-box-keyword-and-boxnew
    

    println!("pub articles vraci: {}", get_author(&articles,1));
    println!("boxed pub articles vraci {}", boxed(&articles,1));
    println!("pub articles reference vraci: {}", get_author_ref(&articles,1));

}
