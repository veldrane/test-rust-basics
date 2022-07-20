#### Notes

* neni mozna pouzivat zaroven methody z impl a zaroven std library i kdyz jsou objekty stejneho typu - viz priklad v 05

    let mut articles = articles::Articles::new();
    articles.add(&article1);
    articles.push(article1.clone()); // tohle sfajluje i kdyz jsou articles vec!

    

#### Important resources

* Rust concurency cheat sheat: https://github.com/quambene/rust-concurrency
Not only about rust but about the concurency programming in general!

* Data types:
    ```Box<T> is for single ownership.
        Rc<T> is for multiple ownership.
        Arc<T> is for multiple ownership, but threadsafe.
        Cell<T> is for "interior mutability" for Copy types; that is, when you need to mutate something behind a &T.
    ```

* Box vs arc vs rc vs ...: https://manishearth.github.io/blog/2015/05/27/wrapper-types-in-rust-choosing-your-guarantees/

* Vec description in deep: https://doc.rust-lang.org/std/vec/struct.Vec.html#guarantees

* Visialisation ownership and borrowing: https://github.com/rustviz/rustviz

* Step by step: https://dev.to/ghost/rust-project-structure-example-step-by-step-3ee
About crates modules etc

* Closures and function: https://zhauniarovich.com/post/2020/2020-12-closures-in-rust/

excelent article about %subj% !


* Basics of the async programming (ehh little bit assembly :)
https://cfsamson.gitbook.io/green-threads-explained-in-200-lines-of-rust/
https://cfsamson.github.io/books-futures-explained/introduction.html

* The dyn keyword - nice explanation of the oop as well
[200~https://www.educative.io/answers/what-is-the-dyn-keyword-in-rust
