#[derive(Clone)]
pub struct Address {
    pub street:     String,
    pub number:     String,
    pub city:       String,
}

#[derive(Clone)]
pub struct User {
    pub username:   String,
    pub id:         i16,
    pub address:    Address,
}

pub struct UserList<'user> {
    pub users: Vec<&'user User>
}


impl User {

    pub fn new_example() -> User {

        let u: User = User {
            username: String::from("Example"),
            id: 123,
            address: Address { street: "Na Horizontu".to_string(),
                 number: "471".to_string(),
                 city: "Exampluv Dvur".to_string()
                 }
        };
        return  u;
    }

    pub fn change_address(&mut self, s: String) -> bool {
        match s.as_str() {
            "Brno" => {
                println!("User {} will not live in fucking {}", self.username, s);
                return false
            }
            &_ => println!("Spravny mesto {}, zadny Brno ! :)", s)
        }
        self.address.city = s;
        return true;
    }
}

impl <'user> UserList<'user> {

    pub fn new() -> UserList<'user> {
        UserList { users: vec![] }
    }

    pub fn add_user_to_list(&mut self, u: &'user User) -> () {
        self.users.push(&u)

    }
}