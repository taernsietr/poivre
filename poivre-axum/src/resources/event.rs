use leptos::ServerFnError;
use std::collections::HashMap;
use std::{
    error::Error,
    fmt
};
use chrono::NaiveDate;
use crate::resources::shared::{
    AttendeeStatus,
    AttendeeStatus::Removed,
    PoivreCard
};

#[derive(Clone)]
struct EventInvite {
  creator: String,
  attendees: HashMap<String, AttendeeStatus>,
  schedule_date: NaiveDate,
  description: String,
}

impl EventInvite {
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
    self.schedule_date = new_date;
  }

  fn update_description(&mut self, new_description: String) {
    self.description = new_description;
  }
}

impl PoivreCard for EventInvite {
    fn url(&self) -> String {
        todo!()
    }

    fn img(&self) -> String {
        todo!()
    }

    fn alt_text(&self) -> String {
        todo!()
    }

    fn card_name(&self) -> String {
        todo!()
    }
}
