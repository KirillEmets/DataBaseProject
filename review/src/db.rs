use postgres::{ Client, NoTls, row::Row, types::ToSql };

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
  pub teacher: String,
  pub subject: String,
  pub owner: String,
  pub text: String,
  pub mark: i16
}

pub struct Db {
  pub client: Client
}

impl Db {
  pub fn new(params: &str) -> Db {
    let client = Client::connect(params, NoTls)
      .expect("Something went wrong with connection to db");
    Db {
      client
    }
  }

  pub fn execute(&mut self, query: &str, params: &[&(dyn ToSql + Sync)]) -> Vec<Row> {
    self.client.query(query, params)
      .expect("Something went wrong with execution of query")
  }
}

