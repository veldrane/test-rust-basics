mod simple;

//use crate::simple::simple::{User,Address};

fn main() {

//    let a: simple::Address = simple::Address {
//        street: "Na Horizontu".to_string(),
//        number: "471".to_string(),
//        city:   "Kraluv Dvur".to_string()
//    };

    let mut u: simple::User = simple::User {
        username: String::from("Honza"),
        id: 123,
        address: simple::Address { street: "Na Horizontu".to_string(),
             number: "471".to_string(),
             city: "Kraluv Dvur".to_string()
             }
    };

    let p: simple::User = simple::User {
        username: String::from("Honza"),
        id: 123,
        address: simple::Address { street: "Na Horizontu".to_string(),
             number: "471".to_string(),
             city: "Kraluv Dvur".to_string()
             }
    };

    let r: simple::User = simple::User {
        username: String::from("Petr"),
        id: 123,
        address: simple::Address { street: "Na Horizontu".to_string(),
             number: "471".to_string(),
             city: "Kraluv Dvur".to_string()
             }
    };

    let mut result = u.change_address("Brno".to_string());
    println!("Username {} bydli ve meste {}", &u.username, &u.address.city);
    result = u.change_address("Praha".to_string());
    println!("Username {} bydli ve meste {}", &u.username, &u.address.city);
    
    u = simple::User::new_example();

    println!("Username {} bydli ve meste {}", &u.username, &u.address.city);

    let mut ul: simple::UserList = simple::UserList::new();

    ul.add_user_to_list(u.clone());
    ul.add_user_to_list(r.clone());
    ul.add_user_to_list(r.clone());
    

  //  let u = u.clone();
  //  ex = u.add_user_to_users(&mut v);

    for k in ul.users.iter() {
        println!("Ahoj from loop for uzivatel: {}", k.username);
    }

}
