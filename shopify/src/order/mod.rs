use crate::client::{Client, Method};
use crate::result::*;
use crate::types::{DateTime, Utc};

mod types;
pub use self::types::*;
mod fulfillment_order;
pub use self::fulfillment_order::*;
mod fulfillment;
pub use self::fulfillment::*;

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

  fn get_risks(&self, order_id: i64) -> ShopifyResult<Vec<OrderRisk>>;

  fn get_fulfillment_orders(&self, order_id: i64) -> ShopifyResult<Vec<FulfillmentOrder>>;

  fn move_fulfillment_order(
    &self,
    fulfillment_order_id: i64,
    move_fulfillment_order: &MoveFulfillmentOrderRequest,
  ) -> ShopifyResult<MoveFulfillmentOrderResponse>;

  fn create_fulfillment(
    &self,
    fulfillment: &CreateFulfillmentRequest,
  ) -> ShopifyResult<Fulfillment>;

  fn update_fulfillment_tracking(
    &self,
    fulfillment_id: i64,
    tracking_info: &TrackingInfo,
    notify_customer: bool,
  ) -> ShopifyResult<Fulfillment>;
}

impl OrderApi for Client {
  fn get_list(&self, params: &GetOrderListParams) -> ShopifyResult<Vec<Order>> {
    shopify_wrap! {
      pub struct Res {
        orders: Vec<Order>,
      }
    }

    let res: Res = self.request_with_params(
      Method::GET,
      "/admin/orders.json",
      params,
      std::convert::identity,
    )?;
    Ok(res.into_inner())
  }

  fn get(&self, id: i64) -> ShopifyResult<Order> {
    shopify_wrap! {
      pub struct Res {
        order: Order,
      }
    }

    let res: Res = self.request(
      Method::GET,
      &format!("/admin/orders/{}.json", id),
      std::convert::identity,
    )?;
    Ok(res.into_inner())
  }

  fn get_risks(&self, order_id: i64) -> ShopifyResult<Vec<OrderRisk>> {
    shopify_wrap! {
      pub struct Res {
        risks: Vec<OrderRisk>,
      }
    }
    let path = format!("/admin/orders/{order_id}/risks.json", order_id = order_id);
    let res: Res = self.request(Method::GET, &path, std::convert::identity)?;
    Ok(res.into_inner())
  }

  fn get_fulfillment_orders(&self, order_id: i64) -> ShopifyResult<Vec<FulfillmentOrder>> {
    shopify_wrap! {
      pub struct Res {
        fulfillment_orders: Vec<FulfillmentOrder>,
      }
    }
    let path = format!(
      "/admin/api/2023-07/orders/{order_id}/fulfillment_orders.json",
      order_id = order_id,
    );
    let res: Res = self.request(Method::GET, &path, std::convert::identity)?;
    Ok(res.into_inner())
  }

  fn move_fulfillment_order(
    &self,
    fulfillment_order_id: i64,
    move_fulfillment_order: &MoveFulfillmentOrderRequest,
  ) -> ShopifyResult<MoveFulfillmentOrderResponse> {
    let path = format!(
      "/admin/api/2023-04/fulfillment_orders/{id}/move.json",
      id = fulfillment_order_id,
    );
    let res: MoveFulfillmentOrderResponse = self.request(Method::POST, &path, move |b| {
      b.json(&serde_json::json!({
        "fulfillment_order": move_fulfillment_order,
      }))
    })?;
    Ok(res)
  }

  fn create_fulfillment(
    &self,
    fulfillment: &CreateFulfillmentRequest,
  ) -> ShopifyResult<Fulfillment> {
    shopify_wrap! {
      pub struct Res {
        fulfillment: Fulfillment,
      }
    }
    let res: Res = self.request(
      Method::POST,
      "/admin/api/2023-04/fulfillments.json",
      move |b| {
        b.json(&serde_json::json!({
          "fulfillment": fulfillment,
        }))
      },
    )?;
    Ok(res.into_inner())
  }

  fn update_fulfillment_tracking(
    &self,
    fulfillment_id: i64,
    tracking_info: &TrackingInfo,
    notify_customer: bool,
  ) -> ShopifyResult<Fulfillment> {
    shopify_wrap! {
      pub struct Res {
        fulfillment: Fulfillment,
      }
    }
    let res: Res = self.request(
      Method::POST,
      &format!(
        "/admin/api/2023-01/fulfillments/{}/update_tracking.json",
        fulfillment_id
      ),
      move |b| {
        b.json(&serde_json::json!({
          "fulfillment": {
            "tracking_info": tracking_info,
            "notify_customer": notify_customer,
          }
        }))
      },
    )?;
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
    use chrono::TimeZone;
    use serde_json::{self, Value};
    use std::fs::File;

    shopify_wrap! {
      pub struct RawOrders {
        orders: Vec<Value>,
      }
    }

    let client = crate::client::get_test_client();
    let mut params = GetOrderListParams::default();
    params.limit = Some(250);
    params.status = Some("any".to_owned());
    params.created_at_min = Some(Utc.ymd(2021, 5, 6).and_hms(18, 11, 0));
    let mut page = 1;
    loop {
      println!("Downloading page {}", page);

      let orders = client
        .request_with_params::<_, RawOrders, _>(
          Method::GET,
          "/admin/orders.json",
          &params,
          std::convert::identity,
        )
        .unwrap()
        .into_inner();

      if orders.is_empty() {
        break;
      }

      let id = orders.last().unwrap().get("id").unwrap().as_i64().unwrap();
      println!("count = {}, last_id = {}", orders.len(), id);

      let f = File::create(format!("{}/order_{}.json", TMP_DIR, page)).unwrap();
      serde_json::to_writer_pretty(f, &orders).unwrap();

      page = page + 1;
      ::std::thread::sleep_ms(500);
    }
  }

  #[test]
  #[ignore]
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
}
