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

    let response = client.post(format!("{}/crates", common::APP_HOST))
        .json(&json!({
            "rustacean_id": rustacean["id"],
            "code": "foo",
            "name": "Foo",
            "version": "0.1",
            "description": "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Pellentesque gravida aliquet arcu, non maximus urna iaculis et. Nam eu ante eu dolor volutpat maximus. Sed tincidunt pretium elementum. Quisque rutrum ex id sem luctus rhoncus ac ultrices lacus. Ut vulputate magna facilisis dignissim porttitor. Nulla vitae pretium neque. Vestibulum rutrum semper justo, ut mattis diam. Curabitur a tempus felis. Pellentesque sit amet pharetra nunc. Curabitur est nunc, tincidunt sit amet arcu sed, bibendum accumsan ligula. Maecenas eu dolor sed mi viverra congue. Phasellus vel dignissim lacus, vel tempor velit. Vestibulum vulputate sapien nisi, ac ullamcorper enim sodales vitae. Aliquam erat volutpat. Etiam tincidunt aliquet velit ac vulputate. Aenean et augue dolor.        Phasellus molestie nisi mi, ut varius dui tempor a. Etiam porta nibh commodo sem efficitur convallis. Sed eget tempor justo, facilisis congue ipsum. Duis cursus cursus convallis. Cras tincidunt maximus urna. Suspendisse faucibus quam elit, hendrerit egestas justo fermentum ut. Interdum et malesuada fames ac ante ipsum primis in faucibus. Etiam vitae augue ac tellus varius pharetra. Duis tempus ante mauris, et pulvinar tellus tempor sed. Maecenas ultrices ante nec nisi maximus, nec maximus est condimentum. Morbi dignissim vitae tortor ac consequat. Nullam rhoncus, nulla in interdum commodo, augue purus elementum magna, vel vehicula libero mauris ac libero. In urna nunc, facilisis vitae porta sed, pulvinar eu odio.        Etiam egestas tortor diam, ac finibus augue varius vel. Etiam bibendum est id enim mattis fermentum. Morbi tincidunt lectus sapien, at dapibus ligula condimentum sed. Vestibulum sed neque eget mauris commodo venenatis ac et orci. Phasellus consectetur diam mollis risus commodo, quis maximus sapien mollis. Duis vitae nisi bibendum, dictum urna fermentum, tempus mi. Donec quis velit nec justo sagittis vehicula eu id sapien.        In tincidunt volutpat nulla a pulvinar. Morbi sed quam et magna feugiat iaculis aliquam a massa. Integer tincidunt mattis tincidunt. Nullam vulputate, lectus eget mollis tincidunt, nisl arcu aliquam tortor, et dignissim metus est eget sapien. Vestibulum eget euismod purus. Phasellus sed consequat purus, tincidunt eleifend felis. Curabitur pharetra metus augue. Nunc at lorem sed mi mattis ornare eu nec leo. Praesent lectus nunc, laoreet tristique pulvinar non, volutpat a lacus. Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Aenean tempus ipsum in elit eleifend feugiat. Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Ut vel consectetur sapien. Suspendisse potenti.        Proin maximus orci eget nibh faucibus, a molestie libero maximus. Sed venenatis rhoncus neque, a egestas nulla ultricies ac. Pellentesque et sodales nisl, et gravida nisl. Nam aliquam nibh at diam elementum, sit amet commodo lorem pretium. Morbi tristique enim sit amet nunc congue lobortis. Sed vel mi est. Aliquam in nibh ac nulla vestibulum euismod.",
        }))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let b_crate: Value = response.json().unwrap();

    assert_eq!(
        b_crate,
        json!({
            "id": b_crate["id"],
            "rustacean_id": rustacean["id"],
            "code": "foo",
            "name": "Foo",
            "version": "0.1",
            "description": "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Pellentesque gravida aliquet arcu, non maximus urna iaculis et. Nam eu ante eu dolor volutpat maximus. Sed tincidunt pretium elementum. Quisque rutrum ex id sem luctus rhoncus ac ultrices lacus. Ut vulputate magna facilisis dignissim porttitor. Nulla vitae pretium neque. Vestibulum rutrum semper justo, ut mattis diam. Curabitur a tempus felis. Pellentesque sit amet pharetra nunc. Curabitur est nunc, tincidunt sit amet arcu sed, bibendum accumsan ligula. Maecenas eu dolor sed mi viverra congue. Phasellus vel dignissim lacus, vel tempor velit. Vestibulum vulputate sapien nisi, ac ullamcorper enim sodales vitae. Aliquam erat volutpat. Etiam tincidunt aliquet velit ac vulputate. Aenean et augue dolor.        Phasellus molestie nisi mi, ut varius dui tempor a. Etiam porta nibh commodo sem efficitur convallis. Sed eget tempor justo, facilisis congue ipsum. Duis cursus cursus convallis. Cras tincidunt maximus urna. Suspendisse faucibus quam elit, hendrerit egestas justo fermentum ut. Interdum et malesuada fames ac ante ipsum primis in faucibus. Etiam vitae augue ac tellus varius pharetra. Duis tempus ante mauris, et pulvinar tellus tempor sed. Maecenas ultrices ante nec nisi maximus, nec maximus est condimentum. Morbi dignissim vitae tortor ac consequat. Nullam rhoncus, nulla in interdum commodo, augue purus elementum magna, vel vehicula libero mauris ac libero. In urna nunc, facilisis vitae porta sed, pulvinar eu odio.        Etiam egestas tortor diam, ac finibus augue varius vel. Etiam bibendum est id enim mattis fermentum. Morbi tincidunt lectus sapien, at dapibus ligula condimentum sed. Vestibulum sed neque eget mauris commodo venenatis ac et orci. Phasellus consectetur diam mollis risus commodo, quis maximus sapien mollis. Duis vitae nisi bibendum, dictum urna fermentum, tempus mi. Donec quis velit nec justo sagittis vehicula eu id sapien.        In tincidunt volutpat nulla a pulvinar. Morbi sed quam et magna feugiat iaculis aliquam a massa. Integer tincidunt mattis tincidunt. Nullam vulputate, lectus eget mollis tincidunt, nisl arcu aliquam tortor, et dignissim metus est eget sapien. Vestibulum eget euismod purus. Phasellus sed consequat purus, tincidunt eleifend felis. Curabitur pharetra metus augue. Nunc at lorem sed mi mattis ornare eu nec leo. Praesent lectus nunc, laoreet tristique pulvinar non, volutpat a lacus. Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Aenean tempus ipsum in elit eleifend feugiat. Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Ut vel consectetur sapien. Suspendisse potenti.        Proin maximus orci eget nibh faucibus, a molestie libero maximus. Sed venenatis rhoncus neque, a egestas nulla ultricies ac. Pellentesque et sodales nisl, et gravida nisl. Nam aliquam nibh at diam elementum, sit amet commodo lorem pretium. Morbi tristique enim sit amet nunc congue lobortis. Sed vel mi est. Aliquam in nibh ac nulla vestibulum euismod.",
            "created_at": b_crate["created_at"]
        })
    );

    common::delete_test_crate(&client, a_crate);
    common::delete_test_crate(&client, b_crate);
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
