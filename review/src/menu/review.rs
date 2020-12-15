use dialoguer::{
  Input
};
use super::*;

pub fn review_menu<'a>() -> std::io::Result<&'a str> {
  const BACK: &str = "I want back to main menu";

  let mut teachers = get_teachers(); 
  teachers.push("New one");
  teachers.push(BACK);

  let selected_teacher = match make_choice(teachers, "Who's teacher?")? {
    "New one" => {
      new_teacher()?
    },
    BACK => {
      return Ok("Back")
    },
    option => option
  };

  let mut subjects = get_subjects(); 
  subjects.push("New one");
  subjects.push(BACK);

  let selected_subject = match make_choice(subjects, "What's the subject?")? {
    "New one" => {
      new_subject()?
    },
    BACK => {
      return Ok("Back")
    },
    option => option
  };
  
  let review: String = Input::new()
    .with_prompt("Write your review")
    .default("leave empty to return to the main menu".into())
    .interact_text()?;

  match review.as_str() {
    "leave empty to return to the main menu" => Ok("Back"),
    r => { 
      post(selected_teacher, selected_subject, r)?;
      Ok("Post")
    }
  }
}

fn new_teacher<'a>() -> std::io::Result<&'a str> {

  todo!()
}

fn new_subject<'a>() -> std::io::Result<&'a str> {

  todo!()
}

fn get_teachers<'a>() -> Vec<&'a str> {
  vec!["some", "idiots"]
}

fn get_subjects<'a>() -> Vec<&'a str> {
  vec!["some", "bullshit", "subjects"]
}

pub fn post(teacher: &str, subject: &str, review: &str) -> std::io::Result<()> {

  todo!()
}