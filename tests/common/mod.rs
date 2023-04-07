use reqwest::{
    blocking::{Client, ClientBuilder},
    header::{self, HeaderMap, HeaderValue},
    StatusCode,
};
use serde_json::{json, Value};
use std::process::Command;

pub static APP_HOST: &'static str = "http://127.0.0.1:8000";

pub fn create_test_rustacean(client: &Client) -> Value {
    let response = client
        .post(format!("{}/rustaceans", APP_HOST))
        .json(&json!({
            "email": "foo@bar.com",
            "name": "Foo",
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    response.json().unwrap()
}

pub fn delete_test_rustacean(client: &Client, rustacean: Value) {
    let response = client
        .delete(format!("{}/rustaceans/{}", APP_HOST, rustacean["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

pub fn create_test_crate(client: &Client, rustacean: &Value) -> Value {
    let response = client
        .post(format!("{}/crates", APP_HOST))
        .json(&json!({
            "rustacean_id": rustacean["id"],
            "code": "foo",
            "name": "Foo",
            "version": "0.1",
            "description": "foo bar",
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    response.json().unwrap()
}

pub fn delete_test_crate(client: &Client, a_crate: Value) {
    let response = client
        .delete(format!("{}/crates/{}", APP_HOST, a_crate["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

fn get_login_for_user(username: &str, role: &str) -> String {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("cli")
        .arg("users")
        .arg("create")
        .arg(username)
        .arg("1234")
        .arg(role)
        .output();
    println!("{:?}", output);

    let client = Client::new();
    let response = client
        .post(format!("{}/login", APP_HOST))
        .json(&json!({
            "username": username,
            "password": "1234",
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let json: Value = response.json().unwrap();
    assert!(json.get("token").is_some());

    format!("Bearer {}", json["token"].as_str().unwrap())
}

pub fn get_client_with_logged_in_admin() -> Client {
    let header_value = get_login_for_user("test_admin", "admin");
    let mut headers = HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        HeaderValue::from_str(header_value.as_str()).unwrap(),
    );
    ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap()
}

pub fn get_client_with_logged_in_editor() -> Client {
    let header_value = get_login_for_user("test_editor", "editor");
    let mut headers = HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        HeaderValue::from_str(header_value.as_str()).unwrap(),
    );
    ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap()
}

pub fn get_client_with_logged_in_viewer() -> Client {
    let header_value = get_login_for_user("test_viewer", "viewer");
    let mut headers = HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        HeaderValue::from_str(header_value.as_str()).unwrap(),
    );
    ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap()
}
