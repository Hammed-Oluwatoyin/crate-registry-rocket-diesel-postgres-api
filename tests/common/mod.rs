use std::process::Command;
use reqwest::{blocking::{Client, ClientBuilder}, StatusCode, header::{HeaderMap, self, HeaderValue}};
use serde_json::{json, Value};

pub static APP_HOST: &'static str = "http://127.0.0.1:8000";

pub fn create_test_rustacean(client: &Client) -> Value {
    let response = client.post(format!("{}/rustaceans", APP_HOST))
        .json(&json!({
            "name": "Foo bar",
            "email": "foo@bar.com"
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

   response.json().unwrap()
}

pub fn create_test_crate(client: &Client, rustacean: &Value) -> Value {
    let response = client.post(format!("{}/crates", APP_HOST))
        .json(&json!({
            "rustacean_id": rustacean["id"],
            "code": "foo",
            "name": "Foo crate",
            "version": "0.1",
            "description": "Foo crate description"
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

   response.json().unwrap()
}

pub fn delete_test_rustacean(client: &Client, rustacean: Value) {
    let response = client.delete(format!("{}/rustaceans/{}", APP_HOST, rustacean["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

pub fn delete_test_crate(client: &Client, a_crate: Value) {
    let response = client.delete(format!("{}/crates/{}", APP_HOST, a_crate["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

pub fn get_client_with_logged_in_admin() -> Client {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("cli")
        .arg("users")
        .arg("create")
        .arg("test_admin")
        .arg("1234")
        .arg("admin")
        .output()
        .unwrap();

    println!("{:?}", output);

    let client = Client::new();
    let response = client.post(format!("{}/login", APP_HOST))
        .json(&json!({
            "username": "test_admin",
            "password": "1234"
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let json: Value = response.json().unwrap();
    assert!(json.get("token").is_some());
    let header_value = format!("Bearer {}", json["token"].as_str().unwrap());

    let mut headers = HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        HeaderValue::from_str(&header_value).unwrap()
    );
    ClientBuilder::new().default_headers(headers).build().unwrap()
}
