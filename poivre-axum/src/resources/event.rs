use std::{
  collections::{
    HashMap,
    hash_map::Iter
  },
  error::Error,
  fmt
};
use leptos::ServerFnError;
use surrealdb::sql::Thing;
use chrono::NaiveDate;
use crate::resources::shared::{
    AttendeeStatus,
    AttendeeStatus::Removed,
    Image,
    Displayable
};

#[derive(Clone)]
struct EventInvite {
  id: Option<Thing>,
  image: Image,
  name: String,
  creator: String,
  attendees: HashMap<String, AttendeeStatus>,
  scheduled_date: NaiveDate,
  description: String,
}

impl EventInvite {
  fn name(&self) -> String { self.name.clone() }
  fn scheduled_date(&self) -> NaiveDate { self.scheduled_date }
  fn creator(&self) -> String { self.creator.clone() }
  fn attendees(&self) -> Iter<'_, String, AttendeeStatus> { self.attendees.iter() }
  fn invites(&self) -> usize { self.attendees.len() }
  fn description(&self) -> String { self.description.clone() }

  fn update_guest_status(&mut self, guest_id: String, status: AttendeeStatus) {
    match self.attendees.get(&guest_id) {
      Some(guest) => { let _ = self.attendees.insert(guest_id, status); },
      None => {
          eprintln!("Guest ID not found")
      },
    };
  }

  fn add_guest(&mut self, guest_id: String) {
    self.attendees.insert(guest_id, AttendeeStatus::Pending);
  }

  fn update_date(&mut self, new_date: NaiveDate) {
    self.scheduled_date = new_date;
  }

  fn update_description(&mut self, new_description: String) {
    self.description = new_description;
  }
}

impl Displayable for EventInvite {
  fn id(&self) -> String {
    match &self.id {
      Some(id) => id.clone().id.to_string(),
      None => String::new()
    }
  }

  fn url(&self) -> String { format!("/events/{}", self.id()) }

  fn image(&self) -> String {
    match &self.image {
      Image::HasImage(url) => url.clone(),
      Image::NoImage => String::new()
    }
  }

  fn display_name(&self) -> String { self.name() }
  fn alt_text(&self) -> String { format!("image for event {}", self.name()) }

  fn headers() -> impl Iterator<Item = String> {
    vec!(
      "ID",
      "Image",
      "Event Name",
      "Creator",
      "Invites",
      "Scheduled Date",
      "Description"
    ).into_iter().map(|x| x.to_string())
  }

  fn row_values(&self) -> impl Iterator<Item = String> {
    vec!(
      self.id(),
      self.image(),
      self.name(),
      self.creator(),
      self.invites().to_string(),
      self.scheduled_date().to_string(),
      self.description()
    ).into_iter()
  }
}
