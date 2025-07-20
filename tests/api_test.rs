const SERVER_ADDRESS: &str = "http://localhost:8000";

#[tokio::test]
async fn test_room_route() {
    let client = reqwest::Client::new();
    let res = client
        .post(format!("{}/room", SERVER_ADDRESS))
        .send()
        .await
        .unwrap();

    assert!(res.status().is_success())
}
