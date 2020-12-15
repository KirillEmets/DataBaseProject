use postgres::{Client, Error, NoTls, row::Row, types::ToSql};

pub struct User {
  pub id: i32,
  pub name: String,
  pub password: String
}

pub struct Teacher {
  pub id: i32,
  pub name: String
}

pub struct Subject {
  pub id: i32,
  pub name: String
}

pub struct Review {
  pub id: i32,
  pub teacher: Teacher,
  pub subject: Subject,
  pub owner: User,
  pub text: String,
  pub mark: u8
}

pub struct Db {
  pub client: Client
}

impl Db {
  pub fn new(params: &str) -> Result<Db, Error> {
    let mut client = Client::connect(params, NoTls)?;
    let db = Db {
      client
    };
    println!("IT's OK");
    Ok(db)
  }

  pub fn execute(&mut self, query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<Vec<Row>, Error> {
    let result = self.client.query(query, params)?;
    Ok(result)
  }
}

