#[macro_use] extern crate rocket;

use std::env;

use anyhow::Result;

use rocket::State;
use rocket::http::Status;

use sqlx::mysql::MySqlPool;



pub struct Receipe {
  pub id: i64,
  pub name: String
}

#[get("/")]
async fn index(pool: &rocket::State<MySqlPool>) -> anyhow::Result<()> {
    
    let recs = list_todos(&pool).await?;
    for rec in recs {
        println!(
            "-  {}: {}",
            rec.id,
            &rec.name,
        );
    }
    Ok(())
}

async fn list_todos(pool: &MySqlPool) -> anyhow::Result<()> {
    let recs = sqlx::query!(
        r#"
SELECT id, name
FROM receipe
ORDER BY id
        "#
    )
    .fetch_all(pool)
    .await;
    return(recs);
}


#[rocket::main]
async fn main() -> anyhow::Result<()>{

    let pool = MySqlPool::connect("mysql://root:@localhost:3306/test").await?;

    rocket::build()
        .mount("/", routes![index])
        .manage(pool)
        .launch()
        .await?;

    Ok(())
}