use dialoguer::Input;
use super::*;
use crate::db::*;

fn new_teacher<'a>(db: &mut Db) -> String {
  let teacher_name: String = Input::new()
    .with_prompt("Write teacher's name")
    // .default("leave empty to return back".into())
    .interact().unwrap();

  db.execute(
    "INSERT INTO teachers (name) VALUES ($1)", 
    &[&teacher_name]
  );

  return teacher_name;
}

fn new_subject<'a>(db: &mut Db) -> String {
  let subject_name: String = Input::new()
    .with_prompt("Write subject's name")
    // .default("leave empty to return back".into())
    .interact().unwrap();

    db.execute(
      "INSERT INTO subjects (name) VALUES ($1)", 
      &[&subject_name]
    );

  return subject_name;
}

pub fn get_teachers(db: &mut Db) -> Vec<Teacher> {
  db.execute(
    "SELECT * FROM teachers", &[]
  )
  .iter()
  .map(|row| Teacher {
    id: row.get(0),
    name: row.get(1)
  })
  .collect()
}

pub fn get_subjects(db: &mut Db) -> Vec<Subject> {
  db.execute(
    "SELECT * FROM subjects", &[]
  )
  .iter()
  .map(|row| Subject {
    id: row.get(0),
    name: row.get(1)
  })
  .collect()
}

pub fn get_reviews(db: &mut Db) -> Vec<Review> {
  db.execute(
    "SELECT * FROM reviews", &[]
  )
  .iter()
  .map(|row| Review {
    id: row.get(0),
    teacher: row.get(1),
    subject: row.get(2),
    owner: row.get(3),
    text: row.get(4),
    mark: row.get(5)
  })
  .collect()
}


fn post(teacher: &str, subject: &str, review: &str, owner: &str, mark: i16, db: &mut Db) {
  db.execute(
    "INSERT INTO reviews(teacher, subject, owner, text, mark) VALUES ($1, $2, $3, $4, $5)", 
    &[&teacher, &subject, &owner, &review, &mark]
  );
}

pub fn review(db: &mut Db, owner: &str) -> Option<MenuInput> {
  const NEW: &str = "New one";
  const BACK: &str = "I want back to menu";

  let teachers = get_teachers(db);

  let mut teachers_option_list: Vec<&str> = teachers
    .iter()
    .map(|teacher| teacher.name.as_str())
    .collect(); 
  teachers_option_list.push(NEW);
  teachers_option_list.push(BACK);

  let selected_teacher = match make_choice(teachers_option_list, "Who's a teacher?").unwrap() {
    NEW => new_teacher(db),
    BACK => return Some(Back),
    option => String::from(option)
  };

  let subjects = get_subjects(db);
  let mut subjects_option_list: Vec<&str> = subjects
    .iter()
    .map(|subject| subject.name.as_str())
    .collect(); 
  subjects_option_list.push(NEW);
  subjects_option_list.push(BACK);

  let selected_subject = match make_choice(subjects_option_list, "What's a subject?").unwrap() {
    NEW => new_subject(db),
    BACK => return Some(Back),
    option => String::from(option)
  };
  
  let review: String = Input::new()
    .with_prompt("Write your review")
    .default("leave empty to return to the main menu".into())
    .interact().unwrap();

  let mark;
  loop {
    let input: String = Input::new()
      .with_prompt("Enter your mark")
      .default("leave empty to return to the main menu".into())
      .interact().unwrap();

    match input.parse::<i16>() {
      Ok(value @ 1..=5) => {
        mark = value;
        break;
      },
      _ => println!("Please enter number between 1 and 5"),
    };
  }

  match review.as_str() {
    "leave empty to return to the main menu" => (),
    r => post(&selected_teacher, &selected_subject, r, owner, mark, db)
  };

  Some(Back)
}