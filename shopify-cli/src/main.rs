use clap::Parser;
use serde_json;
use shopify::client::Client;
use std::env::var;

#[derive(Parser)]
struct Opts {
  #[clap(short, long, default_value = ".env")]
  config: String,
  #[clap(subcommand)]
  subcmd: SubCommand,
}

#[derive(Parser)]
enum SubCommand {
  ProductList,
  VariantList,
  OrderList,
  OrderGet(OrderGet),
  OrderGetRisks(OrderGet),
  OrderGetFulfillmentOrders(OrderGet),
  OrderFulfill(OrderFulfill),
  LocationList,
}

#[derive(Parser)]
struct OrderGet {
  id: i64,
}

#[derive(Parser)]
struct OrderFulfill {
  id: i64,
  item_id: i64,
  location_id: i64,
  carrier: String,
  tracking_number: String,
}

fn main() {
  let opts: Opts = Opts::parse();

  dotenv::from_filename(&opts.config).unwrap();
  let client = Client::new(
    &var("SHOPIFY_BASE_URL").unwrap(),
    &var("SHOPIFY_API_KEY").unwrap(),
    &var("SHOPIFY_PASSWORD").unwrap(),
  )
  .unwrap();

  match opts.subcmd {
    SubCommand::ProductList => product_list(&client),
    SubCommand::VariantList => variant_list(&client),
    SubCommand::OrderGet(OrderGet { id }) => order_get(&client, id),
    SubCommand::OrderGetRisks(OrderGet { id }) => order_get_risks(&client, id),
    SubCommand::OrderGetFulfillmentOrders(OrderGet { id }) => {
      order_get_fulfillment_orders(&client, id)
    }
    SubCommand::OrderList => order_list(&client),
    SubCommand::OrderFulfill(fulfill) => order_fulfill(&client, &fulfill),
    SubCommand::LocationList => location_list(&client),
  }
}

fn product_list(client: &Client) {
  use shopify::product::*;

  let mut all = vec![];

  let page = client
    .list(&GetProductListParams {
      ..Default::default()
    })
    .unwrap();

  let mut next = page.get_next();

  all.extend(page.payload);

  loop {
    if let Some(params) = next {
      let page = client.list_page(&params).unwrap();
      next = page.get_next();

      all.extend(page.payload);
    } else {
      break;
    }
  }

  serde_json::to_writer_pretty(std::io::stdout(), &all).unwrap()
}

fn variant_list(client: &Client) {
  use shopify::variant::*;

  let mut all = vec![];

  let page = client
    .list(&GetVariantListParams {
      ..Default::default()
    })
    .unwrap();

  let mut next = page.get_next();

  all.extend(page.payload);

  loop {
    if let Some(params) = next {
      let page = client.list_page(&params).unwrap();
      next = page.get_next();

      all.extend(page.payload);
    } else {
      break;
    }
  }

  serde_json::to_writer_pretty(std::io::stdout(), &all).unwrap()
}

fn order_get(client: &Client, id: i64) {
  use shopify::order::*;

  let order = client.get(id).unwrap();

  serde_json::to_writer_pretty(std::io::stdout(), &order).unwrap()
}

fn order_get_risks(client: &Client, id: i64) {
  use shopify::order::*;

  let risks = client.get_risks(id).unwrap();

  serde_json::to_writer_pretty(std::io::stdout(), &risks).unwrap()
}

fn order_get_fulfillment_orders(client: &Client, id: i64) {
  use shopify::order::*;

  let risks = client.get_fulfillment_orders(id).unwrap();

  serde_json::to_writer_pretty(std::io::stdout(), &risks).unwrap()
}

fn order_list(client: &Client) {
  use shopify::order::*;

  #[derive(serde::Deserialize)]
  struct P {
    orders: Vec<Order>,
  }
  let orders: P = serde_json::from_reader(std::fs::File::open("1.json").unwrap()).unwrap();

  let orders = client.get_list(&Default::default()).unwrap();

  serde_json::to_writer_pretty(std::io::stdout(), &orders).unwrap()
}

fn order_fulfill(client: &Client, fulfill: &OrderFulfill) {
  use shopify::order::*;

  let fos = client.get_fulfillment_orders(fulfill.id).unwrap();
  dbg!(&fos);
  let (fo, li) = fos
    .iter()
    .filter(|fo| fo.status == FulfillmentOrderStatus::Open)
    .filter_map(|fo| {
      if let Some(li) = fo
        .line_items
        .iter()
        .find(|li| li.line_item_id == fulfill.item_id)
      {
        Some((fo, li))
      } else {
        None
      }
    })
    .next()
    .expect("No matching fulfillment order");
  if fo.assigned_location_id != Some(fulfill.location_id) {
    println!(
      "Moving fulfillment order {} to location {}",
      fo.id, fulfill.location_id
    );
    client
      .move_fulfillment_order(
        fo.id,
        &MoveFulfillmentOrderRequest {
          new_location_id: fulfill.location_id,
          fulfillment_order_line_items: None,
        },
      )
      .unwrap();
  }

  let r = client
    .create_fulfillment(&CreateFulfillmentRequest {
      line_items_by_fulfillment_order: vec![LineItemsByFulfillmentOrder {
        fulfillment_order_id: fo.id,
        fulfillment_order_line_items: vec![FulfillmentOrderLineItems {
          id: li.id,
          quantity: 1,
        }],
      }],
      notify_customer: Some(true),
      tracking_info: Some(TrackingInfo {
        number: fulfill.tracking_number.clone(),
        company: fulfill.carrier.clone(),
        url: None,
      })
    })
    .unwrap();

  serde_json::to_writer_pretty(std::io::stdout(), &r).unwrap()
}

fn location_list(client: &Client,) {
  use shopify::inventory::LocationApi;

  serde_json::to_writer_pretty(std::io::stdout(), &client.get_list().unwrap()).unwrap()
}