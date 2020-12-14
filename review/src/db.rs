use postgres::{Client, Error, NoTls};

struct User {
  id: i32,
  name: String,
  password: String
}

struct Teacher {
  id: i32,
  name: String
}

struct Subject {
  id: i32,
  name: String
}

struct Review {
  id: i32,
  teacher: Teacher,
  subject: Subject,
  owner: User,
  text: String,
  mark: u8
}

struct Db {
  client: Client
}

// "postgresql://postgres:postgres@127.0.0.1/rust",
// NoTls,

// ("SELECT * FROM SystemUser", &[])? {
//   let user = User {
//       id: row.get(0),
//       name: row.get(1),
//       password: row.get(2)
//   };
//   println!("{} {} {}", user.id, user.name, user.password);
// }

impl Db {
  fn new<T>(params: &str, tls_mode: T) -> Result<Db, Error> {
    let mut client = Client::connect(params, tls_mode)?;
    let db = Db {
      client
    };
    Ok(db)
  }

  fn execute(&self, query: &str) -> Result<Vec<Row>, Error> {
    let result = self.client.query(query, &[])?
    Ok(result)
  }
}

