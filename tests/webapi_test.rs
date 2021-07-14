use actix_web::{test, web, App};
use qdrant::{index, VersionInfo};

#[actix_rt::test]
async fn test_index_status() {
    // Given app and request related to index api
    let mut app = test::init_service(App::new().service(index)).await;
    let req = test::TestRequest::default().to_request();

    // When call index
    let resp = test::call_service(&mut app, req).await;

    // Then status must be ok
    assert!(resp.status().is_success());
}

#[actix_rt::test]
async fn test_index_json() {
    // Given app and request related to index api
    let mut app = test::init_service(App::new().service(index)).await;
    let req = test::TestRequest::get().uri("/").to_request();

    // And the expected  answer
    let expected = VersionInfo {
        title: "qdrant - vector search engine".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    };

    // When call index
    let resp: VersionInfo = test::read_response_json(&mut app, req).await;

    // Then answer must be the expected
    assert_eq!(resp, expected);
}
