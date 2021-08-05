use std::ops::DerefMut;

use crate::db::Pool;
use async_graphql::*;
use serde::{Deserialize, Serialize};
use sqlx::{self, Row, postgres::PgRow};
use strum_macros::{Display, EnumString};
pub struct QueryRoot;

#[derive(
  Clone, Copy, Debug, Deserialize, Display, Enum, Eq, EnumString, PartialEq, Serialize, sqlx::Type,
)]
#[sqlx(type_name = "chocolate_type")]
pub enum ChocolateType {
  Bitter,
  White,
  Milk,
}

#[derive(SimpleObject, sqlx::FromRow)]
#[graphql(complex)]
#[derive(Debug)]
struct Product {
  id: i32,
  name: String,
  price: i32,
  chocolate_type: ChocolateType,
  fillings: Vec<String>,
  images: Vec<String>
}

#[ComplexObject]
impl Product {
  // pub async fn friends(&self, ctx: &Context<'_>) -> FieldResult<Vec<Character>> {
  //   let pool = ctx.data::<Pool>().unwrap();
  //   let query_str = format!(
  //     r#"SELECT id, name, kind FROM starwars.characters
  //     JOIN starwars.friends ON starwars.friends.friend_id = id AND
  //     starwars.friends.character_id = {}"#,
  //     self.id
  //   );
  //   let result = sqlx::query_as::<_, Character>(query_str.as_str())
  //     .fetch_all(pool.get().await.unwrap().deref_mut())
  //     .await
  //     .unwrap();
  //   return Ok(result);
  // }
}

#[Object]
impl QueryRoot {
  async fn products(&self, ctx: &Context<'_>) -> FieldResult<Vec<Product>> {
    let pool = ctx.data::<Pool>().unwrap();
    let query_str = format!("select id, name, price, chocolate_type, fillings, images from products");
    let result = sqlx::query(query_str.as_str())
      .map(|row: PgRow| Product {
        id: row.get("id"),
        name: row.get("name"),
        price: row.get("price"),
        chocolate_type: row.get("chocolate_type"),
        fillings: row.get("fillings"),
        images: row.get("images"),
      })
      .fetch_all(pool.get().await.unwrap().deref_mut())
      .await
      .unwrap();
    return Ok(result);
  }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_product(
        &self, 
        ctx: &Context<'_>, 
        name: String, 
        price: i32, 
        chocolate_type: ChocolateType,
        fillings: Vec<String>,
        images: Vec<String>,
    ) -> ID {
        let pool = ctx.data::<Pool>().unwrap();
        let query_str = format!("insert into products(name, price, chocolate_type, fillings, images) 
            values ('{}', {}, '{:?}', '{:?}', '{:?}') returning id", name, price, chocolate_type, fillings, images);
        let query_str = query_str.replace("[", "{").replace("]", "}"); // handles arrays formatting

        let result: Result<i32, sqlx::Error> = sqlx::query(query_str.as_str())
            .map(|row: PgRow| row.get("id") )
            .fetch_one(pool.get().await.unwrap().deref_mut())
            .await;
 
        let mut id: i32 = -1;    
        match result {
          Ok(v) => {id = v; }
          Err(e) => println!("Error creating new product: {}", e)
        }

        ID::from(id)
    }

    async fn update_product_price(
      &self, 
      ctx: &Context<'_>, 
      id: ID,
      new_price: i32, 
    ) -> Result<bool> {
        let pool = ctx.data::<Pool>().unwrap();
        let query_str = format!("update products set price = {} where id = {}", new_price, String::from(id));

        let result = sqlx::query(query_str.as_str())
            .execute(pool.get().await.unwrap().deref_mut())
            .await;

        match result {
           Ok(v) => { Ok(v.rows_affected() > 0) },
           Err(e) => {println!("Error while updating: {}", e); Ok(false) }
        }
    }

    async fn delete_product(&self, ctx: &Context<'_>, id: ID) -> Result<bool> {
        let pool = ctx.data::<Pool>().unwrap();
        let query_str = format!("delete from products where id = {}", String::from(id));

        let result = sqlx::query(query_str.as_str())
              .execute(pool.get().await.unwrap().deref_mut())
              .await;

        match result {
          Ok(v) => { Ok(v.rows_affected() > 0) },
          Err(e) => {println!("Error while deleting: {}", e); Ok(false) }
        }
    }
}