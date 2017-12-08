use error::*;
use client::{Client, Method};
use types::{DateTime, Utc};

mod types;
pub use self::types::*;
mod fulfillment;
pub use self::fulfillment::NewFulfillment;

pub struct OrderApi<'a> {
  client: &'a Client,
}

impl Client {
  pub fn order(&self) -> OrderApi {
    OrderApi { client: self }
  }
}

request_query! {
  pub struct GetOrderListParams {
    pub ids: Option<Vec<i64>>,
    pub limit: Option<i64>,
    pub page: Option<i64>,
    pub since_id: Option<i64>,
    pub created_at_min: Option<DateTime<Utc>>,
    pub created_at_max: Option<DateTime<Utc>>,
    pub updated_at_min: Option<DateTime<Utc>>,
    pub updated_at_max: Option<DateTime<Utc>>,
    pub processed_at_min: Option<DateTime<Utc>>,
    pub processed_at_max: Option<DateTime<Utc>>,
    pub attribution_app_id: Option<String>,
    pub status: Option<String>,
    pub financial_status: Option<String>,
    pub fulfillment_status: Option<String>,
    pub fields: Option<Vec<String>>,
  }
}

impl<'a> OrderApi<'a> {
  pub fn get_list(&self, params: &GetOrderListParams) -> Result<Vec<Order>> {
    shopify_wrap! {
      pub struct Res {
        orders: Vec<Order>,
      }
    }

    let res: Res = self.client.request_with_params(
      Method::Get,
      "/admin/orders.json",
      params,
      |_| {},
    )?;
    Ok(res.into_inner())
  }

  pub fn create_fulfillment(
    &self,
    order_id: i64,
    fulfillment: NewFulfillment,
  ) -> Result<Fulfillment> {
    shopify_wrap! {
      pub struct Res {
        fulfillment: Fulfillment,
      }
    }
    let path = format!("/admin/orders/{}/fulfillments.json", order_id);
    let res: Res = self.client.request(Method::Post, &path, move |b| {
      b.json(&json!({
        "fulfillment": fulfillment
      }));
    })?;
    Ok(res.into_inner())
  }

  pub fn update_fulfillment(
    &self,
    order_id: i64,
    fulfillment_id: i64,
    fulfillment: NewFulfillment,
  ) -> Result<Fulfillment> {
    shopify_wrap! {
      pub struct Res {
        fulfillment: Fulfillment,
      }
    }
    let path = format!(
      "/admin/orders/{order_id}/fulfillments/{fulfillment_id}.json",
      order_id = order_id,
      fulfillment_id = fulfillment_id
    );
    let res: Res = self.client.request(Method::Put, &path, move |b| {
      b.json(&json!({
        "fulfillment": fulfillment
      }));
    })?;
    Ok(res.into_inner())
  }

  pub fn complete_fulfillment(&self, order_id: i64, fulfillment_id: i64) -> Result<Fulfillment> {
    shopify_wrap! {
      pub struct Res {
        fulfillment: Fulfillment,
      }
    }
    let path = format!(
      "/admin/orders/{order_id}/fulfillments/{fulfillment_id}/complete.json",
      order_id = order_id,
      fulfillment_id = fulfillment_id
    );
    let res: Res = self.client.request(Method::Post, &path, |_| {})?;
    Ok(res.into_inner())
  }

  pub fn open_fulfillment(&self, order_id: i64, fulfillment_id: i64) -> Result<Fulfillment> {
    shopify_wrap! {
      pub struct Res {
        fulfillment: Fulfillment,
      }
    }
    let path = format!(
      "/admin/orders/{order_id}/fulfillments/{fulfillment_id}/open.json",
      order_id = order_id,
      fulfillment_id = fulfillment_id
    );
    let res: Res = self.client.request(Method::Post, &path, |_| {})?;
    Ok(res.into_inner())
  }

  pub fn cancel_fulfillment(&self, order_id: i64, fulfillment_id: i64) -> Result<Fulfillment> {
    shopify_wrap! {
      pub struct Res {
        fulfillment: Fulfillment,
      }
    }
    let path = format!(
      "/admin/orders/{order_id}/fulfillments/{fulfillment_id}/cancel.json",
      order_id = order_id,
      fulfillment_id = fulfillment_id
    );
    let res: Res = self.client.request(Method::Post, &path, |_| {})?;
    Ok(res.into_inner())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  const TMP_DIR: &'static str = "./tmp/ventray";

  #[test]
  #[ignore]
  fn test_dump_all_orders() {
    use std::fs::File;
    use std::io::Write;
    use serde_json::{self, Value};
    use chrono::TimeZone;

    shopify_wrap! {
      pub struct RawOrders {
        orders: Vec<Value>,
      }
    }

    let client = ::client::get_test_client();
    let mut params = GetOrderListParams::default();
    params.limit = Some(250);
    params.status = Some("any".to_owned());
    let mut page = 1;
    loop {
      println!("Downloading page {}", page);

      let orders = client
        .request_with_params::<_, RawOrders, _>(Method::Get, "/admin/orders.json", &params, |_| {})
        .unwrap()
        .into_inner();

      if orders.is_empty() {
        break;
      }

      let id = orders.last().unwrap().get("id").unwrap().as_i64().unwrap();
      println!("count = {}, last_id = {}", orders.len(), id);

      params.page = Some(page + 1);

      let f = File::create(format!("{}/order_{}.json", TMP_DIR, page)).unwrap();
      serde_json::to_writer_pretty(f, &orders).unwrap();

      page = page + 1;
      ::std::thread::sleep_ms(500);
    }
  }

  #[test]
  // #[ignore]
  fn test_deserialize_all() {
    use std::io;
    use std::io::Write;
    use std::fs;
    use serde_json::{self, Value};

    let mut chunk = 1;
    loop {
      let f = match fs::File::open(format!("{}/order_{}.json", TMP_DIR, chunk)) {
        Ok(f) => f,
        Err(ref err) if err.kind() == io::ErrorKind::NotFound => {
          break;
        }
        Err(ref err) => panic!("io err: {}", err),
      };

      println!("testing chunk {}", chunk);

      let orders: Vec<Value> = serde_json::from_reader(f).unwrap();

      let total = orders.len();
      for (i, order) in orders.into_iter().enumerate() {
        let id = order.get("id").unwrap().as_i64().unwrap();
        let as_str = serde_json::to_string_pretty(&order).unwrap();
        let mut current = fs::File::create(format!("{}/current_order.json", TMP_DIR)).unwrap();
        write!(&mut current, "{}", &as_str).unwrap();
        let order: Order = serde_json::from_str(&as_str).unwrap();
        println!("testing order {}: {} of {}", id, i + 1, total);
      }

      chunk = chunk + 1;
    }
  }

  #[test]
  #[ignore]
  fn test_create_fulfillment() {
    let client = ::client::get_test_client();
    let mut f = NewFulfillment::new();
    f.add_item(59878440973, Some(1)).tracking_number("7777");
    client.order().create_fulfillment(33673216013, f).unwrap();
  }

  #[test]
  fn test_update_fulfillment() {
    let client = ::client::get_test_client();
    let mut f = NewFulfillment::new();
    f.add_item(59878440973, Some(1)).tracking_number(
      "1Z30434EDG37750543",
    );
    client
      .order()
      .update_fulfillment(33673216013, 34429861901, f)
      .unwrap();
  }
}