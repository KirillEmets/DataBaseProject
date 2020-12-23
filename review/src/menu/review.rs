use dialoguer::Input;
use postgres::Error;
use postgres::types::ToSql;
use std::result::Result;
use super::*;
use crate::db::*;


fn only_numbers(s: String) -> bool {
  for c in s.chars() {
    if !"0123456789".contains(c) {
      return false;
    }
  }
  true
}

fn new_teacher<'a>(db: &mut Db) -> std::io::Result<&'a str> {
  let teacher_name: String = Input::new()
  .with_prompt("Write teacher's name")
  // .default("leave empty to return back".into())
  .interact_text()?;

  todo!()
}

fn new_subject<'a>(db: &mut Db) -> std::io::Result<&'a str> {
  let subject_name: String = Input::new()
    .with_prompt("Write subject's name")
    // .default("leave empty to return back".into())
    .interact_text()?;

  todo!()
}

pub fn get_teachers(db: &mut Db) -> Vec<Teacher> {
  let teachers = db.execute(
    "SELECT * FROM teachers", &[]
  )
  .iter()
  .map(|row| Teacher {
    id: row.get(0),
    name: row.get(1)
  })
  .collect();
 teachers
}

pub fn get_subjects(db: &mut Db) -> Vec<Subject>{
  let subjects = db.execute(
    "SELECT * FROM subjects", &[]
  )
  .iter()
  .map(|row| Subject {
    id: row.get(0),
    name: row.get(1)
  })
  .collect();
  subjects
}

fn post(teacher: &str, subject: &str, review: &str, owner: &str, mark: i64, db: &mut Db) {
  let query = format!("INSERT INTO reviews VALUES ($1, $2, $3, $4, {})", mark);
  db.execute(
    "INSERT INTO reviews VALUES ($1, $2, $3, $4, $5)", &[&teacher, &subject, &owner, &review, &mark]
  );
}

pub fn review(db: &mut Db, owner: &str) -> Option<MenuInput> {
  const BACK: &str = "I want back to main menu";

  let teachers = get_teachers(db);

  let mut teachers_option_list: Vec<&str> = teachers
    .iter()
    .map(|teacher| teacher.name.as_str())
    .collect(); 
  teachers_option_list.push("New one");
  teachers_option_list.push(BACK);

  let selected_teacher = match make_choice(teachers_option_list, "Who's a teacher?").unwrap() {
    "New one" => {
      new_teacher(db).unwrap()
    },
    BACK => {
      return Some(Back)
    },
    option => option
  };

  let subjects = get_subjects(db);
  let mut subjects_option_list: Vec<&str> = subjects
    .iter()
    .map(|subject| subject.name.as_str())
    .collect(); 
  subjects_option_list.push("New one");
  subjects_option_list.push(BACK);

  let selected_subject = match make_choice(subjects_option_list, "What's a subject?").unwrap() {
    "New one" => {
      new_subject(db).unwrap()
    },
    BACK => {
      return Some(Back)
    },
    option => option
  };
  
  let review: String = Input::new()
    .with_prompt("Write your review")
    .default("leave empty to return to the main menu".into())
    .interact_text().unwrap();

  let mark;
  loop {
    let input: String = Input::new()
      .with_prompt("Enter your mark")
      .default("leave empty to return to the main menu".into())
      .interact_text().unwrap();
    match input.parse::<i64>() {
      Ok(value) => {
        mark = value;
        break;
      },
      Err(_) => {}
    };
  }

  match review.as_str() {
    "leave empty to return to the main menu" => (),
    r => { 
      post(selected_teacher, selected_subject, r, owner, mark, db);
    }
  }

  Some(Back)
}