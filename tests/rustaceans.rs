use reqwest::{blocking::Client, StatusCode};
use serde_json::{json, Value};

fn create_test_rustacean(client: &Client) -> Value {
    let response = client
        .post("http://127.0.0.1:8000/rustaceans")
        .json(&json!({
            "email": "foo@bar.com",
            "name": "Foo",
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    response.json().unwrap()
}
fn delete_test_rustacean(client: &Client, rustacean: Value) {
    let response = client
        .delete(format!(
            "http://127.0.0.1:8000/rustaceans/{}",
            rustacean["id"]
        ))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

#[test]
fn test_get_rustaceans() {
    let client = Client::new();
    let response = client
        .get("http://127.0.0.1:8000/rustaceans")
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let rustacean1 = create_test_rustacean(&client);

    let rustacean2 = create_test_rustacean(&client);

    let response = client
        .get("http://127.0.0.1:8000/rustaceans")
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let response_json: Value = response.json().unwrap();

    assert!(response_json.as_array().unwrap().contains(&rustacean1));
    assert!(response_json.as_array().unwrap().contains(&rustacean2));

    delete_test_rustacean(&client, rustacean1);
    delete_test_rustacean(&client, rustacean2);
}

#[test]
fn test_create_rustacean() {
    let client = Client::new();
    let response = client
        .post("http://127.0.0.1:8000/rustaceans")
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

    delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_view_rustacean() {
    let client = Client::new();
    let rustacean = create_test_rustacean(&client);

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

    delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_update_rustacean() {
    let client = Client::new();
    let rustacean = create_test_rustacean(&client);

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
    delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_delete_rustacean() {
    let client = Client::new();
    let json = create_test_rustacean(&client);

    let response = client
        .delete(format!("http://127.0.0.1:8000/rustaceans/{}", json["id"]))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}
