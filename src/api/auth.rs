use crate::{ApiError, ApiResult};
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use axum::routing::post;
use tracing;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::FromRow;


pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

#[tracing::instrument]
async fn api_login(payload: Json<LoginPayload>) -> ApiResult<Json<Value>> {
    tracing::info!("->> {:<12} - api_login", "HANDLER");

    // TODO : find a way to make the pool and Create table
    //          global to avoid reuse this code in other files

    // Set db related variable to build the connection string
    let db_type = "mariadb";
    let db_name = "test_auth";
    let db_username = "root"; // TODO : change this
    let db_host = "localhost";
    let db_pwd = "root"; // TODO : change this
    let db_string = format!("{}://{}:{}@{}/{}", db_type, db_username, db_pwd, db_host, db_name);

    // Create a connection pool
    let pool = MySqlPoolOptions::new()
                    .max_connections(5)
                    // connect is : "'dbName'://'username':'password'@'hostname'/'database_to_accesss'"
                    .connect("mariadb://root:root@localhost/test_auth") //.connect(&db_string)
                    .await?;

    // Create a table if not exists yet
    sqlx::query(
            r#"
                CREATE TABLE IF NOT EXISTS users(
                    userid      int NOT NULL AUTO_INCREMENT,
                    username    varchar(100) NOT NULL UNIQUE,
                    pwd         varchar(100) NOT NULL,
                    PRIMARY KEY (UserID)
                );
            "#,
    )
    .execute(&pool)
    .await?;
    
    let select_query = sqlx::query_as::<_, User>("SELECT username, pwd FROM users");
    let users: Vec<User> = select_query
        .fetch_all(&pool)
        .await?;
        //.filter(|user| user.username == payload.username);
    
    for user in users {
        if user.username != payload.username {
            continue;
        }

        if user.pwd != payload.pwd {
            return Err(ApiError::LoginFail);
        }
    }


    // TODO: Implement real db/auth logic.
    /*if payload.username != "demo1" || payload.pwd != "welcome" {
        return Err(ApiError::LoginFail);
    }*/

    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,
}

#[derive(Debug, FromRow)]
struct User {
    username: String,
    pwd: String,
}
