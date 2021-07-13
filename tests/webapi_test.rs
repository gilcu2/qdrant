use actix_web::{test, web, App};
use qdrant::index;

#[actix_rt::test]
async fn test_index_get() {
    let mut app = test::init_service(App::new().route("/", web::get().to(index))).await;
}
