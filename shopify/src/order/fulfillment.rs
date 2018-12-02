#[derive(Debug, Default, Serialize)]
pub struct NewFulfillment {
  tracking_company: Option<String>,
  tracking_number: Option<String>,
  tracking_numbers: Option<Vec<String>>,
  tracking_url: Option<String>,
  notify_customer: Option<bool>,
  line_items: Vec<Item>,
  location_id: Option<i64>,
}

#[derive(Debug, Default, Serialize)]
struct Item {
  id: i64,
  quantity: Option<i64>,
}

impl NewFulfillment {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn location_id(&mut self, id: i64) -> &mut Self {
    self.location_id = Some(id);
    self
  }

  pub fn tracking_number<T: Into<String>>(&mut self, value: T) -> &mut Self {
    let value: String = value.into();
    self.tracking_number = Some(value.clone());
    self.tracking_numbers = Some(vec![value]);
    self
  }

  pub fn tracking_numbers<T>(&mut self, values: T) -> &mut Self
  where
    T: IntoIterator,
    <T as IntoIterator>::Item: Into<String>,
  {
    self.tracking_number = None;
    self.tracking_numbers = Some(values.into_iter().map(Into::into).collect());
    self
  }

  pub fn tracking_company<T: Into<String>>(&mut self, value: T) -> &mut Self {
    self.tracking_company = Some(value.into());
    self
  }

  pub fn tracking_url<T: Into<String>>(&mut self, value: T) -> &mut Self {
    self.tracking_url = Some(value.into());
    self
  }

  pub fn notify_customer(&mut self, value: bool) -> &mut Self {
    self.notify_customer = Some(value);
    self
  }

  pub fn add_item(&mut self, id: i64, quantity: Option<i64>) -> &mut Self {
    match self.line_items.iter().position(|i| i.id == id) {
      Some(pos) => self.line_items[pos].quantity = quantity,
      None => {
        self.line_items.push(Item {
          id: id,
          quantity: quantity,
        });
      }
    };
    self
  }
}
