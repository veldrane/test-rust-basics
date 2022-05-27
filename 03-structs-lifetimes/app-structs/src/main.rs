mod simple;

//use crate::simple::simple::{User,Address};

fn main() {

    let mut u1: simple::User = simple::User {
        username: String::from("Kreten"),
        id: 123,
        address: simple::Address { street: "Na Horizontu".to_string(),
             number: "471".to_string(),
             city: "Kraluv Dvur".to_string()
             }
    };

    let u2: simple::User = simple::User {
        username: String::from("Honza"),
        id: 123,
        address: simple::Address { street: "Na Horizontu".to_string(),
             number: "471".to_string(),
             city: "Kraluv Dvur".to_string()
             }
    };

    let u3: simple::User = simple::User {
        username: String::from("Petr"),
        id: 123,
        address: simple::Address { street: "Na Horizontu".to_string(),
             number: "471".to_string(),
             city: "Kraluv Dvur".to_string()
             }
    };

    let mut ul: simple::UserList = simple::UserList::new();

    u1.change_address("Pragl".to_string());
    u1.change_address("Cosi".to_string());

    ul.add_user_to_list(&u1);

    let mut ux = u1.clone();
    ux.change_address("Prdlacka".to_string());


    ul.add_user_to_list(&u1);
    ul.add_user_to_list(&ux);
    ul.add_user_to_list(&u1);
    ul.add_user_to_list(&u2);
    ul.add_user_to_list(&u3);

    for k in ul.users.iter() {
        println!("Ahoj from loop for uzivatel: {}", k.username);
        println!("User {} bydli v {}", k.username, k.address.city)
    }

}
