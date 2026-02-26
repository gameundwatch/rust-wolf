use actix_web::{test, web, App};
use backend::{AppState, Event, MessagePayload, handle_get_events, handle_post_events};
use std::sync::Mutex;

// TODO: lib.rs に handle_get_events, handle_post_events を移動して pub で公開した後、
// ここで以下のようにインポートする
//
// 現状ハンドラは main.rs にあるためテストクレートから参照できず、
// .service() に登録できていないことが全テスト 404 の原因
//
// また main.rs の handle_post_events は HttpResponse::Created() (201) を返しているが、
// テストは 200 を期待している。どちらかに揃える必要がある

/// テスト用の共有ステートを作成
fn create_test_app_state() -> web::Data<AppState> {
    web::Data::new(AppState {
        events: Mutex::new(Vec::new()),
    })
}

/// 初期状態で GET /api/events は空配列を返す
#[actix_web::test]
async fn test_get_events_empty() {
    let state = create_test_app_state();
    let app = test::init_service(
        App::new()
            .app_data(state.clone())
            .service(handle_get_events)
    )
    .await;

    let req = test::TestRequest::get().uri("/api/events").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 200);

    let body: Vec<Event> = test::read_body_json(resp).await;
    assert!(body.is_empty());
}

/// POST /api/events でイベントを追加し、追加されたイベントが返る
#[actix_web::test]
async fn test_post_event() {
    let state = create_test_app_state();
    let app = test::init_service(
        App::new()
            .app_data(state.clone())
            .service(handle_post_events)
    )
    .await;

    let event = Event {
        event_type: "message".to_string(),
        time: "2026-02-24T12:00:00Z".to_string(),
        from: "User".to_string(),
        message: Some(MessagePayload {
            to: "all".to_string(),
            content: "Hello, world!".to_string(),
        }),
    };

    let req = test::TestRequest::post()
        .uri("/api/events")
        .set_json(&event)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 201);
}

/// POST した後に GET すると追加したイベントが含まれる
#[actix_web::test]
async fn test_get_events_after_post() {
    let state = create_test_app_state();
    let app = test::init_service(
        App::new()
            .app_data(state.clone())
            .service(handle_post_events)
            .service(handle_get_events)
    )
    .await;

    let event = Event {
        event_type: "message".to_string(),
        time: "2026-02-24T12:00:00Z".to_string(),
        from: "Player1".to_string(),
        message: Some(MessagePayload {
            to: "all".to_string(),
            content: "誰が人狼？".to_string(),
        }),
    };

    // まず POST
    let req = test::TestRequest::post()
        .uri("/api/events")
        .set_json(&event)
        .to_request();
    test::call_service(&app, req).await;

    // GET で確認
    let req = test::TestRequest::get().uri("/api/events").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 200);

    let body: Vec<Event> = test::read_body_json(resp).await;
    assert_eq!(body.len(), 1);
    assert_eq!(body[0].from, "Player1");
    assert_eq!(body[0].message.as_ref().unwrap().content, "誰が人狼？");
}

/// 不正な JSON を POST すると 400 が返る
#[actix_web::test]
async fn test_post_event_invalid_body() {
    let state = create_test_app_state();
    let app = test::init_service(
        App::new()
            .app_data(state.clone())
            .service(handle_post_events)
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/events")
        .set_payload("not valid json")
        .insert_header(("content-type", "application/json"))
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 400);
}
