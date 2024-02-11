use crate::client::{Client, Method};
use crate::result::*;

mod types;
pub use self::types::*;

pub trait LocationApi {
  fn get_list(&self) -> ShopifyResult<Vec<Location>>;
  fn get(&self, id: i64) -> ShopifyResult<Location>;
}

impl LocationApi for Client {
  fn get_list(&self) -> ShopifyResult<Vec<Location>> {
    shopify_wrap! {
      pub struct Res {
        locations: Vec<Location>,
      }
    }

    let res: Res = self.request(Method::GET, "/admin/locations.json", std::convert::identity)?;
    Ok(res.into_inner())
  }

  fn get(&self, id: i64) -> ShopifyResult<Location> {
    shopify_wrap! {
      pub struct Res {
        location: Location,
      }
    }
    let path = format!("/admin/locations/{}.json", id);
    let res: Res = self.request(Method::GET, &path, std::convert::identity)?;
    Ok(res.into_inner())
  }
}

request_query! {
  pub struct GetInventoryLevelsParams {
    pub limit: Option<i64>,
    pub page: Option<i64>,
    pub inventory_item_ids: Option<Vec<i64>>,
    pub location_ids: Option<Vec<i64>>,
  }
}

pub trait InventoryLevelApi {
  fn get_list(&self, params: &GetInventoryLevelsParams) -> ShopifyResult<Vec<InventoryLevel>>;
}

impl InventoryLevelApi for Client {
  fn get_list(&self, params: &GetInventoryLevelsParams) -> ShopifyResult<Vec<InventoryLevel>> {
    shopify_wrap! {
      pub struct Res {
        inventory_levels: Vec<InventoryLevel>,
      }
    }

    let res: Res = self.request_with_params(
      Method::GET,
      "/admin/inventory_levels.json",
      params,
      std::convert::identity,
    )?;
    Ok(res.into_inner())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::inventory;
  use std::env::var;

  fn create_test_client() -> Client {
    dotenv::dotenv().ok();
    Client::new(
      &var("SHOPIFY_BASE_URL").unwrap(),
      &var("SHOPIFY_API_KEY").unwrap(),
      &var("SHOPIFY_PASSWORD").unwrap(),
    )
    .unwrap()
  }
  #[test]
  #[ignore]
  fn test_location_get_list() {
    let client = create_test_client();
    let list = inventory::LocationApi::get_list(&client).unwrap();
    println!("{:#?}", list);
  }

  #[test]
  #[ignore]
  fn test_inventory_level_get_list() {
    use super::GetInventoryLevelsParams;
    let client = create_test_client();
    let list = inventory::InventoryLevelApi::get_list(
      &client,
      &GetInventoryLevelsParams {
        inventory_item_ids: Some(vec![2819391175, 5746930631]),
        ..Default::default()
      },
    )
    .unwrap();
    println!("{:#?}", list);
  }
}
