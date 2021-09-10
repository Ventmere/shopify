use client::{Client, Method};
use result::*;
use types::{DateTime, Utc};

mod types;
pub use self::types::*;
mod fulfillment;
pub use self::fulfillment::NewFulfillment;

request_query! {
  pub struct GetOrderListParams {
    pub ids: Option<Vec<i64>>,
    pub limit: Option<i64>,
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

pub trait OrderApi {
  fn get_list(&self, params: &GetOrderListParams) -> ShopifyResult<Vec<Order>>;

  fn get(&self, id: i64) -> ShopifyResult<Order>;

  fn create_fulfillment(
    &self,
    order_id: i64,
    fulfillment: &NewFulfillment,
  ) -> ShopifyResult<Fulfillment>;

  fn update_fulfillment(
    &self,
    order_id: i64,
    fulfillment_id: i64,
    fulfillment: &NewFulfillment,
  ) -> ShopifyResult<Fulfillment>;

  fn complete_fulfillment(&self, order_id: i64, fulfillment_id: i64) -> ShopifyResult<Fulfillment>;

  fn open_fulfillment(&self, order_id: i64, fulfillment_id: i64) -> ShopifyResult<Fulfillment>;

  fn cancel_fulfillment(&self, order_id: i64, fulfillment_id: i64) -> ShopifyResult<Fulfillment>;
}

impl OrderApi for Client {
  fn get_list(&self, params: &GetOrderListParams) -> ShopifyResult<Vec<Order>> {
    shopify_wrap! {
      pub struct Res {
        orders: Vec<Order>,
      }
    }

    let res: Res = self.request_with_params(Method::Get, "/admin/orders.json", params, |_| {})?;
    Ok(res.into_inner())
  }

  fn get(&self, id: i64) -> ShopifyResult<Order> {
    shopify_wrap! {
      pub struct Res {
        order: Order,
      }
    }

    let res: Res = self.request(Method::Get, &format!("/admin/orders/{}.json", id), |_| {})?;
    Ok(res.into_inner())
  }

  fn create_fulfillment(
    &self,
    order_id: i64,
    fulfillment: &NewFulfillment,
  ) -> ShopifyResult<Fulfillment> {
    shopify_wrap! {
      pub struct Res {
        fulfillment: Fulfillment,
      }
    }
    let path = format!("/admin/orders/{}/fulfillments.json", order_id);
    let res: Res = self.request(Method::Post, &path, move |b| {
      b.json(&json!({ "fulfillment": fulfillment }));
    })?;
    Ok(res.into_inner())
  }

  fn update_fulfillment(
    &self,
    order_id: i64,
    fulfillment_id: i64,
    fulfillment: &NewFulfillment,
  ) -> ShopifyResult<Fulfillment> {
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
    let res: Res = self.request(Method::Put, &path, move |b| {
      b.json(&json!({ "fulfillment": fulfillment }));
    })?;
    Ok(res.into_inner())
  }

  fn complete_fulfillment(&self, order_id: i64, fulfillment_id: i64) -> ShopifyResult<Fulfillment> {
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
    let res: Res = self.request(Method::Post, &path, |_| {})?;
    Ok(res.into_inner())
  }

  fn open_fulfillment(&self, order_id: i64, fulfillment_id: i64) -> ShopifyResult<Fulfillment> {
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
    let res: Res = self.request(Method::Post, &path, |_| {})?;
    Ok(res.into_inner())
  }

  fn cancel_fulfillment(&self, order_id: i64, fulfillment_id: i64) -> ShopifyResult<Fulfillment> {
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
    let res: Res = self.request(Method::Post, &path, |_| {})?;
    Ok(res.into_inner())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  const TMP_DIR: &'static str = "./tmp/ventray";

  #[test]
  fn test_dump_all_orders() {
    use chrono::TimeZone;
    use serde_json::{self, Value};
    use std::fs::File;
    use std::io::Write;

    shopify_wrap! {
      pub struct RawOrders {
        orders: Vec<Value>,
      }
    }

    let client = ::client::get_test_client();
    let mut params = GetOrderListParams::default();
    params.limit = Some(250);
    params.status = Some("any".to_owned());
    params.created_at_min = Some(Utc.ymd(2021, 5, 6).and_hms(18, 11, 0));
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
    use serde_json::{self, Value};
    use std::fs;
    use std::io;
    use std::io::Write;

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
    client.create_fulfillment(33673216013, &f).unwrap();
  }

  #[test]
  fn test_update_fulfillment() {
    let client = ::client::get_test_client();
    let mut f = NewFulfillment::new();
    f.add_item(59878440973, Some(1))
      .tracking_number("1Z30434EDG37750543");
    client
      .update_fulfillment(33673216013, 34429861901, &f)
      .unwrap();
  }
}
