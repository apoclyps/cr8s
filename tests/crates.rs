use reqwest::{blocking::Client, StatusCode};
use serde_json::{json, Value};

pub mod common;

#[test]
fn test_get_crates() {
    let client = Client::new();
    let response = client
        .get(format!("{}/crates", common::APP_HOST))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let rustacean = common::create_test_rustacean(&client);
    let crate1 = common::create_test_crate(&client, &rustacean);
    let crate2 = common::create_test_crate(&client, &rustacean);
    let response = client
        .get(format!("{}/crates", common::APP_HOST))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let response_json: Value = response.json().unwrap();

    assert!(response_json.as_array().unwrap().contains(&crate1));
    assert!(response_json.as_array().unwrap().contains(&crate2));

    common::delete_test_crate(&client, crate1);
    common::delete_test_crate(&client, crate2);
}

#[test]
fn test_create_crate() {
    let client = Client::new();
    let rustacean = common::create_test_rustacean(&client);
    let response = client
        .post(format!("{}/crates", common::APP_HOST))
        .json(&json!({
            "rustacean_id": rustacean["id"],
            "code": "foo",
            "name": "Foo",
            "version": "0.1",
            "description": null,
        }))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let a_crate: Value = response.json().unwrap();

    assert_eq!(
        a_crate,
        json!({
            "id": a_crate["id"],
            "rustacean_id": rustacean["id"],
            "code": "foo",
            "name": "Foo",
            "version": "0.1",
            "description": null,
            "created_at": a_crate["created_at"]
        })
    );

    common::delete_test_crate(&client, a_crate);
}

#[test]
fn test_view_crate() {
    let client = Client::new();
    let rustacean = common::create_test_rustacean(&client);
    let a_crate = common::create_test_crate(&client, &rustacean);

    let response = client
        .get(format!("{}/crates/{}", common::APP_HOST, a_crate["id"]))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let json: Value = response.json().unwrap();

    assert_eq!(
        json,
        json!({
            "id": a_crate["id"],
            "rustacean_id": rustacean["id"],
            "code": "foo",
            "name": "Foo",
            "version": "0.1",
            "description": "foo bar",
            "created_at": a_crate["created_at"]
        })
    );

    common::delete_test_crate(&client, a_crate);
    common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_update_crate() {
    let client = Client::new();
    let rustacean = common::create_test_rustacean(&client);
    let a_crate = common::create_test_crate(&client, &rustacean);

    let response = client
        .put(format!("{}/crates/{}", common::APP_HOST, a_crate["id"]))
        .json(&json!({
            "rustacean_id": rustacean["id"],
            "code": "fooz",
            "name": "Fooz",
            "version": "0.1.1",
            "description": "fooz baz"
        }))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let json: Value = response.json().unwrap();

    assert_eq!(
        json,
        json!({
            "id": a_crate["id"],
            "rustacean_id": rustacean["id"],
            "code": "fooz",
            "name": "Fooz",
            "version": "0.1.1",
            "description": "fooz baz",
            "created_at": a_crate["created_at"]
        })
    );

    common::delete_test_crate(&client, a_crate);
    common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_delete_crate() {
    let client = Client::new();
    let rustacean = common::create_test_rustacean(&client);
    let a_crate = common::create_test_crate(&client, &rustacean);

    let response = client
        .delete(format!("{}/crates/{}", common::APP_HOST, a_crate["id"]))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
    common::delete_test_rustacean(&client, rustacean);
}
