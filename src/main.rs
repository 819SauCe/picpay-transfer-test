use axum::{extract::{State, Json, Path}, routing::{post, get}, Router};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, PgPool};
use sqlx::types::BigDecimal;
use std::env;
use std::str::FromStr;
use dotenvy;
use tokio::net::TcpListener;



#[derive(Deserialize, Serialize, Debug, Clone)]
struct User {
    id: i32,
    user_type: String,
}


#[derive(Deserialize, Serialize, Debug)]
struct Payload {
    payer_id: i32,
    payee_id: i32,
    value: f64,
}

async fn get_user(State(pool): State<PgPool>, Path(id): Path<i32>) -> Json<Option<User>> {
    let user = get_user_by_id(&pool, id).await;
    Json(user)
}


async fn get_user_by_id(pool: &PgPool, user_id: i32) -> Option<User> {
    let user = sqlx::query_as!(
        User,
        "SELECT id, user_type FROM users WHERE id = $1",
        user_id
    )
    .fetch_optional(pool)
    .await
    .unwrap();

    if let Some(ref u) = user {
        println!("User encontrado: {:?}", u);
    } else {
        println!("User não encontrado!");
    }

    user
}


async fn transfer(State(pool): State<PgPool>, Json(payload): Json<Payload>) {
    let value_decimal = BigDecimal::from_str(&payload.value.to_string()).unwrap();
    sqlx::query!(
        "INSERT INTO transactions (payer_id, payee_id, value) VALUES ($1, $2, $3)",
        payload.payer_id,
        payload.payee_id,
        value_decimal
    )
    .execute(&pool)
    .await
    .unwrap();

    println!("transferencia realizada!");
}

#[tokio::main]
async fn main() {
    dotenvy::from_filename(".env").ok();
    let database_url = env::var("DATABASE_URL").expect("ERRO NA VARIAVEL DATABASE_URL");
    let pool = PgPoolOptions::new().max_connections(5).connect(&database_url).await.unwrap();

    let app = Router::new()
        .route("/transfer", post(transfer))
        .route("/user/:id", axum::routing::get(get_user))
        .with_state(pool);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
        
}
