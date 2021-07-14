use actix_web::{test, web, App, http::header};
use qdrant::{index, VersionInfo};
use qdrant::settings::Settings;
use storage::content_manager::toc::TableOfContent;
use qdrant::common::helpers::create_search_runtime;
use qdrant::api::collections_api::{get_collection, get_collections, update_collections};

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

#[actix_rt::test]
async fn test_create_collection() {
    // Given the app
    let settings = Settings::new().expect("Can't read config.");
    let runtime = create_search_runtime(settings.storage.performance.max_search_threads).unwrap();
    let handle = runtime.handle().clone();
    let toc = TableOfContent::new(&settings.storage, handle);
    let toc_data = web::Data::new(toc);
    let mut app = test::init_service(
        App::new()
            .app_data(toc_data.clone())
            .service(update_collections)
    ).await;

    // And the request
    let payload = r#"{
        "create_collection": {
            "name": "test_collection",
            "vector_size": 4,
            "distance": "Dot"
        }
    }"#.as_bytes();

    let req = test::TestRequest::post()
        .uri("/collections")
        .insert_header((header::CONTENT_TYPE, "application/json"))
        .set_payload(payload)
        .to_request();

    // When call
    let resp = test::call_service(&mut app, req).await;

    // Then status must be ok
    assert!(resp.status().is_success());
}
