fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use async_trait::async_trait;
    use test_context::{test_context, AsyncTestContext};
    use wiremock::{
        http::Method,
        matchers::{method, path},
        Mock, MockServer, ResponseTemplate,
    };

    struct MyTestContext {
        url: String,
    }

    #[async_trait]
    impl AsyncTestContext for MyTestContext {
        async fn setup() -> Self {
            let mock_server = MockServer::start().await;
            Mock::given(method(Method::Get))
                .and(path("/test-context-endpoint"))
                .respond_with(ResponseTemplate::new(200).set_body_string(""))
                .mount(&mock_server)
                .await;

            Self {
                url: mock_server.uri(),
            }
        }
    }

    #[test_context(MyTestContext)]
    #[tokio::test]
    async fn test_with_context1(ctx: &MyTestContext) {
        assert_eq!(
            reqwest::get(format!("{}/test-context-endpoint", ctx.url))
                .await
                .unwrap()
                .status(),
            200
        );
    }

    #[test_context(MyTestContext)]
    #[tokio::test]
    async fn test_with_context2(ctx: &MyTestContext) {
        assert_eq!(
            reqwest::get(format!("{}/test-context-endpoint", ctx.url))
                .await
                .unwrap()
                .status(),
            200
        );
    }

    #[test_context(MyTestContext)]
    #[tokio::test]
    async fn test_with_context_special(_ctx: &MyTestContext) {
        let mock_server = MockServer::start().await;
        Mock::given(method(Method::Get))
            .and(path("/test-endpoint"))
            .respond_with(ResponseTemplate::new(200).set_body_string(""))
            .mount(&mock_server)
            .await;

        assert_eq!(
            reqwest::get(format!("{}/test-endpoint", mock_server.uri()))
                .await
                .unwrap()
                .status(),
            200
        );
    }
}
