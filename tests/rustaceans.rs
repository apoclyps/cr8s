use reqwest::{blocking::Client, StatusCode};
use serde_json::{json, Value};

pub mod common;

#[test]
fn test_get_rustaceans() {
    let client = Client::new();
    let response = client
        .get(format!("{}/rustaceans", common::APP_HOST))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let rustacean1 = common::create_test_rustacean(&client);

    let rustacean2 = common::create_test_rustacean(&client);

    let response = client
        .get(format!("{}/rustaceans", common::APP_HOST))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let response_json: Value = response.json().unwrap();

    assert!(response_json.as_array().unwrap().contains(&rustacean1));
    assert!(response_json.as_array().unwrap().contains(&rustacean2));

    common::delete_test_rustacean(&client, rustacean1);
    common::delete_test_rustacean(&client, rustacean2);
}

#[test]
fn test_create_rustacean() {
    let client = Client::new();
    let response = client
        .post(format!("{}/rustaceans", common::APP_HOST))
        .json(&json!({
            "email": "foo@bar.com",
            "name": "Foo",
        }))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let rustacean: Value = response.json().unwrap();

    assert_eq!(
        rustacean,
        json!({
            "id": rustacean["id"],
            "email": "foo@bar.com",
            "name": "Foo",
            "created_at": rustacean["created_at"]
        })
    );

    common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_view_rustacean() {
    let client = Client::new();
    let rustacean = common::create_test_rustacean(&client);

    let response = client
        .get(format!(
            "http://127.0.0.1:8000/rustaceans/{}",
            rustacean["id"]
        ))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let json: Value = response.json().unwrap();

    assert_eq!(
        json,
        json!({
            "id": rustacean["id"],
            "email": "foo@bar.com",
            "name": "Foo",
            "created_at": rustacean["created_at"]
        })
    );

    let unknown_rustacean_id: i32 = 999999;

    let response = client
        .get(format!(
            "{}/rustaceans/{}",
            common::APP_HOST,
            unknown_rustacean_id
        ))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_update_rustacean() {
    let client = Client::new();
    let rustacean = common::create_test_rustacean(&client);

    let response = client
        .put(format!(
            "http://127.0.0.1:8000/rustaceans/{}",
            rustacean["id"]
        ))
        .json(&json!({
            "email": "fooz@bar.com",
            "name": "FooZ",
        }))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let json: Value = response.json().unwrap();

    assert_eq!(
        json,
        json!({
            "id": rustacean["id"],
            "email": "fooz@bar.com",
            "name": "FooZ",
            "created_at": rustacean["created_at"]
        })
    );
    common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_delete_rustacean() {
    let client = Client::new();
    let rustacean = common::create_test_rustacean(&client);

    let response = client
        .delete(format!(
            "http://127.0.0.1:8000/rustaceans/{}",
            rustacean["id"]
        ))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}
