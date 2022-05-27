use std::fmt;


fn printme (a: &str) {
    println!("Ahooj {}", a)
}


fn main() {
    let a: &str = "Honzo";
    let b: &str = a.clone();

    let mut c: i16 = 1234;
    let mut d: String = (c+14).to_string();

    printme(a);
    printme(b);
    printme(&c.to_string());
    printme(&d);


    c = 5432;
    d = "picmund".to_string();


    printme (&c.to_string());
    printme(&d)
}
