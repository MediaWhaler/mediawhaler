use crate::ApiResult;
use axum::{Json, Router};
use axum::routing::post;
use serde::Deserialize;
use serde_json::{json, Value};
use tracing;
use sqlx::mysql::MySqlPoolOptions;


pub fn routes() -> Router {
    Router::new().route("/api/users", post(api_create_user))
}


/*
** @brief : Create a new user in the db
** @param payload : payload containing user's infos 
**                  (username, password) to store in the db
** @return : A json result with the status code
*/
async fn api_create_user(payload: Json<UserPayload>) -> ApiResult<Json<Value>> {
    tracing::info!("->> {:12} - api_create_user", "HANDLER");

    // TODO : find a way to make the pool and Create table
    //          global to avoid reuse this code in other files
    // Create a connection pool
    let pool = MySqlPoolOptions::new()
                    .max_connections(5)
                    // connect is : "'dbName'://'username':'password'@'hostname'/'database_to_accesss'"
                    // TODO : put the db name in a variable
                    .connect("mariadb://root:root@localhost/test_auth")
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
    

    // Insert the new user
    // TODO :   - we want to check here if there are no SQL injection
    //          - we want to hash the password here
    let row : (i64,) = sqlx::query_as("INSERT INTO users (username, pwd) VALUES (?,?) returning
    userid")
        .bind(&payload.username)
        .bind(&payload.pwd)
        .fetch_one(&pool)
        .await?;

    tracing::info!("UserID for the new row : {}", row.0);

    // create the return json to notify that the request succeded
    let body = Json(json!({
        "result": {
            "success": true
        }
    }));
    
    Ok(body)
}

#[derive(Debug, Deserialize)]
struct UserPayload {
    username : String,
    pwd : String,
}

