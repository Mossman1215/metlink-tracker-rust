#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn hello_handles() {
        let request = Request::default();
        let expected = json!({
            "message": "Go Serverless v1.0! Your function executed successfully!"
        })
        .into_response();
        let response = hello(request, Context::default())
            .await
            .expect("expected Ok(_) value")
            .into_response();
        assert_eq!(response.body(), expected.body())
    }
}
