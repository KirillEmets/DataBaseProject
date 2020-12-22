use dialoguer::Input;
use super::*;

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

fn get_teachers<'a>(db: &mut Db) -> Vec<&'a str> {
  vec!["some", "idiots"]
}

fn get_subjects<'a>(db: &mut Db) -> Vec<&'a str> {
  vec!["some", "bullshit", "subjects"]
}

fn post(teacher: &str, subject: &str, review: &str, db: &mut Db) -> std::io::Result<()> {

  todo!()
}

pub fn review(db: &mut Db) -> Option<MenuInput> {
  const BACK: &str = "I want back to main menu";

  let mut teachers_option_list = get_teachers(db); 
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

  let mut subjects_option_list = get_subjects(db); 
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

  match review.as_str() {
    "leave empty to return to the main menu" => (),
    r => { 
      post(selected_teacher, selected_subject, r, db).unwrap();
    }
  }

  Some(Back)
}