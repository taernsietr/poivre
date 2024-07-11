enum GuestStatus {
  Pending,
  Rejected,
  Accepted,
  Removed
}

struct EventInvite {
  creator: String,
  guests: HashMap<String, GuestStatus>,
  schedule_date: NaiveDate,
  description: String,
}

impl EventInvite {
  fn update_guest_status(&self, guest_id: String, status: GuestStatus) {
    match self.guests.get(guest_id) {
      Some(guest) => guest.set_status(status),
      None => eprintln!("Guest ID not found"),
    };
  }

  fn remove_guest(&self, guest_id: String) -> Result<(), Error> {
    match &self.guests.get(guest_id) {
      None => GuestNotInEventError,
      Ok(guest) => {
        guest.set(Removed);
        Ok(())
      }
    }
  }
  fn add_guest(&self, guest_id: String) {
    self.guests.insert(guest_id);
  }
  fn update_date(&self, new_date: NaiveDate) {
    self.schedule_date = new_date;
  }
  fn update_description(&self, new_description: String) {
    self.description = new_description;
  }
}

impl PoivreCard for EventInvite {}
