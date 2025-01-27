use super::*;

#[test]
fn request() {
    let payload: Payload = Payload::Request(Request::new(
        1.into(),
        Params::Publish(Publish {
            topic: "topic".into(),
            message: "payload".into(),
            ttl_secs: 12,
            tag: 0,
            prompt: false,
        }),
    ));

    let serialized = serde_json::to_string(&payload).unwrap();

    assert_eq!(
        &serialized,
        r#"{"id":1,"jsonrpc":"2.0","method":"irn_publish","params":{"topic":"topic","message":"payload","ttl":12,"tag":0}}"#
    );

    let deserialized: Payload = serde_json::from_str(&serialized).unwrap();

    assert_eq!(&payload, &deserialized)
}

#[test]
fn subscribe() {
    let payload: Payload = Payload::Request(Request::new(
        1659980684711969.into(),
        Params::Subscribe(Subscribe {
            topic: "c4163cf65859106b3f5435fc296e7765411178ed452d1c30337a6230138c9840".into(),
        }),
    ));

    let serialized = serde_json::to_string(&payload).unwrap();

    assert_eq!(
        &serialized,
        r#"{"id":1659980684711969,"jsonrpc":"2.0","method":"irn_subscribe","params":{"topic":"c4163cf65859106b3f5435fc296e7765411178ed452d1c30337a6230138c9840"}}"#
    );

    let deserialized: Payload = serde_json::from_str(&serialized).unwrap();

    assert_eq!(&payload, &deserialized)
}

#[test]
fn response_result() {
    let payload: Payload = Payload::Response(Response::Success(SuccessfulResponse::new(
        1.into(),
        "some result".into(),
    )));

    let serialized = serde_json::to_string(&payload).unwrap();

    assert_eq!(
        &serialized,
        r#"{"id":1,"jsonrpc":"2.0","result":"some result"}"#
    );

    let deserialized: Payload = serde_json::from_str(&serialized).unwrap();

    assert_eq!(&payload, &deserialized)
}

#[test]
fn response_error() {
    let payload: Payload =
        Payload::Response(Response::Error(ErrorResponse::new(1.into(), ErrorData {
            code: 32,
            data: None,
            message: "some message".into(),
        })));

    let serialized = serde_json::to_string(&payload).unwrap();

    assert_eq!(
        &serialized,
        r#"{"id":1,"jsonrpc":"2.0","error":{"code":32,"message":"some message"}}"#
    );

    let deserialized: Payload = serde_json::from_str(&serialized).unwrap();

    assert_eq!(&payload, &deserialized)
}

#[test]
fn subscription() {
    let data = SubscriptionData {
        topic: "test_topic".into(),
        message: "test_message".into(),
        published_at: 123,
        tag: 1000,
    };
    let params = Subscription {
        id: "test_id".into(),
        data,
    };
    let payload: Payload = Payload::Request(Request::new(1.into(), Params::Subscription(params)));

    let serialized = serde_json::to_string(&payload).unwrap();

    assert_eq!(
        &serialized,
        r#"{"id":1,"jsonrpc":"2.0","method":"irn_subscription","params":{"id":"test_id","data":{"topic":"test_topic","message":"test_message","publishedAt":123,"tag":1000}}}"#
    );

    let deserialized: Payload = serde_json::from_str(&serialized).unwrap();

    assert_eq!(&payload, &deserialized)
}

#[test]
fn deserialize_iridium_method() {
    let serialized = r#"{"id":1,"jsonrpc":"2.0","method":"iridium_subscription","params":{"id":"test_id","data":{"topic":"test_topic","message":"test_message","publishedAt":123,"tag":1000}}}"#;
    assert!(serde_json::from_str::<'_, Payload>(serialized).is_ok());
}

#[test]
fn deserialize_batch_methods() {
    let serialized = r#"{
        "id" : 1,
        "jsonrpc": "2.0",
        "method": "irn_batchSubscribe",
        "params": {
            "topics": [
                "c4163cf65859106b3f5435fc296e7765411178ed452d1c30337a6230138c9840",
                "c4163cf65859106b3f5435fc296e7765411178ed452d1c30337a6230138c9841"
            ]
        }
    }"#;
    assert_eq!(
        serde_json::from_str::<'_, Payload>(serialized).unwrap(),
        Payload::Request(Request {
            id: 1.into(),
            jsonrpc: "2.0".into(),
            params: Params::BatchSubscribe(BatchSubscribe {
                topics: vec![
                    Topic::from("c4163cf65859106b3f5435fc296e7765411178ed452d1c30337a6230138c9840"),
                    Topic::from("c4163cf65859106b3f5435fc296e7765411178ed452d1c30337a6230138c9841")
                ]
            })
        })
    );

    let serialized = r#"{
        "id" : 1,
        "jsonrpc": "2.0",
        "method": "irn_batchUnsubscribe",
        "params": {
            "subscriptions": [
                {
                    "topic": "c4163cf65859106b3f5435fc296e7765411178ed452d1c30337a6230138c9840",
                    "id": "c4163cf65859106b3f5435fc296e7765411178ed452d1c30337a6230138c9841"
                },
                {
                    "topic": "c4163cf65859106b3f5435fc296e7765411178ed452d1c30337a6230138c9842",
                    "id": "c4163cf65859106b3f5435fc296e7765411178ed452d1c30337a6230138c9843"
                }
            ]
        }
    }"#;
    assert_eq!(
        serde_json::from_str::<'_, Payload>(serialized).unwrap(),
        Payload::Request(Request {
            id: 1.into(),
            jsonrpc: "2.0".into(),
            params: Params::BatchUnsubscribe(BatchUnsubscribe {
                subscriptions: vec![
                    Unsubscribe {
                        topic: Topic::from(
                            "c4163cf65859106b3f5435fc296e7765411178ed452d1c30337a6230138c9840"
                        ),
                        subscription_id: SubscriptionId::from(
                            "c4163cf65859106b3f5435fc296e7765411178ed452d1c30337a6230138c9841"
                        ),
                    },
                    Unsubscribe {
                        topic: Topic::from(
                            "c4163cf65859106b3f5435fc296e7765411178ed452d1c30337a6230138c9842"
                        ),
                        subscription_id: SubscriptionId::from(
                            "c4163cf65859106b3f5435fc296e7765411178ed452d1c30337a6230138c9843"
                        ),
                    }
                ]
            })
        })
    );
}

#[test]
fn validation() {
    // Valid data.
    let id = MessageId::from(1);
    let jsonrpc: Arc<str> = "2.0".into();
    let message: Arc<str> = "0".repeat(512).into();
    let topic = Topic::from("c4163cf65859106b3f5435fc296e7765411178ed452d1c30337a6230138c9840");
    let subscription_id =
        SubscriptionId::from("c4163cf65859106b3f5435fc296e7765411178ed452d1c30337a6230138c9841");

    // Invalid JSONRPC version.
    let request = Request {
        id,
        jsonrpc: "invalid".into(),
        params: Params::Publish(Publish {
            topic: topic.clone(),
            message: message.clone(),
            ttl_secs: 0,
            tag: 0,
            prompt: false,
        }),
    };
    assert_eq!(request.validate(), Err(ValidationError::JsonRpcVersion));

    // Publish: valid.
    let request = Request {
        id,
        jsonrpc: jsonrpc.clone(),
        params: Params::Publish(Publish {
            topic: topic.clone(),
            message: message.clone(),
            ttl_secs: 0,
            tag: 0,
            prompt: false,
        }),
    };
    assert_eq!(request.validate(), Ok(()));

    // Publish: invalid topic.
    let request = Request {
        id,
        jsonrpc: jsonrpc.clone(),
        params: Params::Publish(Publish {
            topic: Topic::from("invalid"),
            message: message.clone(),
            ttl_secs: 0,
            tag: 0,
            prompt: false,
        }),
    };
    assert_eq!(
        request.validate(),
        Err(ValidationError::TopicDecoding(DecodingError::Length))
    );

    // Subscribe: valid.
    let request = Request {
        id,
        jsonrpc: jsonrpc.clone(),
        params: Params::Subscribe(Subscribe {
            topic: topic.clone(),
        }),
    };
    assert_eq!(request.validate(), Ok(()));

    // Subscribe: invalid topic.
    let request = Request {
        id,
        jsonrpc: jsonrpc.clone(),
        params: Params::Subscribe(Subscribe {
            topic: Topic::from("invalid"),
        }),
    };
    assert_eq!(
        request.validate(),
        Err(ValidationError::TopicDecoding(DecodingError::Length))
    );

    // Unsubscribe: valid.
    let request = Request {
        id,
        jsonrpc: jsonrpc.clone(),
        params: Params::Unsubscribe(Unsubscribe {
            topic: topic.clone(),
            subscription_id: subscription_id.clone(),
        }),
    };
    assert_eq!(request.validate(), Ok(()));

    // Unsubscribe: invalid topic.
    let request = Request {
        id,
        jsonrpc: jsonrpc.clone(),
        params: Params::Unsubscribe(Unsubscribe {
            topic: Topic::from("invalid"),
            subscription_id: subscription_id.clone(),
        }),
    };
    assert_eq!(
        request.validate(),
        Err(ValidationError::TopicDecoding(DecodingError::Length))
    );

    // Fetch: valid.
    let request = Request {
        id,
        jsonrpc: jsonrpc.clone(),
        params: Params::FetchMessages(FetchMessages {
            topic: topic.clone(),
        }),
    };
    assert_eq!(request.validate(), Ok(()));

    // Fetch: invalid topic.
    let request = Request {
        id,
        jsonrpc: jsonrpc.clone(),
        params: Params::FetchMessages(FetchMessages {
            topic: Topic::from("invalid"),
        }),
    };
    assert_eq!(
        request.validate(),
        Err(ValidationError::TopicDecoding(DecodingError::Length))
    );

    // Subscription: valid.
    let request = Request {
        id,
        jsonrpc: jsonrpc.clone(),
        params: Params::Subscription(Subscription {
            id: subscription_id.clone(),
            data: SubscriptionData {
                topic: topic.clone(),
                message: message.clone(),
                published_at: 123,
                tag: 1000,
            },
        }),
    };
    assert_eq!(request.validate(), Ok(()));

    // Subscription: invalid subscription ID.
    let request = Request {
        id,
        jsonrpc: jsonrpc.clone(),
        params: Params::Subscription(Subscription {
            id: SubscriptionId::from("invalid"),
            data: SubscriptionData {
                topic: topic.clone(),
                message: message.clone(),
                published_at: 123,
                tag: 1000,
            },
        }),
    };
    assert_eq!(
        request.validate(),
        Err(ValidationError::SubscriptionIdDecoding(
            DecodingError::Length
        ))
    );

    // Subscription: invalid topic.
    let request = Request {
        id,
        jsonrpc: jsonrpc.clone(),
        params: Params::Subscription(Subscription {
            id: subscription_id.clone(),
            data: SubscriptionData {
                topic: Topic::from("invalid"),
                message,
                published_at: 123,
                tag: 1000,
            },
        }),
    };
    assert_eq!(
        request.validate(),
        Err(ValidationError::TopicDecoding(DecodingError::Length))
    );

    // Batch subscription: valid.
    let request = Request {
        id,
        jsonrpc: jsonrpc.clone(),
        params: Params::BatchSubscribe(BatchSubscribe {
            topics: vec![topic.clone()],
        }),
    };
    assert_eq!(request.validate(), Ok(()));

    // Batch subscription: empty list.
    let request = Request {
        id,
        jsonrpc: jsonrpc.clone(),
        params: Params::BatchSubscribe(BatchSubscribe { topics: vec![] }),
    };
    assert_eq!(request.validate(), Err(ValidationError::BatchEmpty));

    // Batch subscription: too many items.
    let topics = (0..MAX_SUBSCRIPTION_BATCH_SIZE + 1)
        .map(|_| Topic::generate())
        .collect();
    let request = Request {
        id,
        jsonrpc: jsonrpc.clone(),
        params: Params::BatchSubscribe(BatchSubscribe { topics }),
    };
    assert_eq!(
        request.validate(),
        Err(ValidationError::BatchLimitExceeded {
            limit: MAX_SUBSCRIPTION_BATCH_SIZE,
            actual: MAX_SUBSCRIPTION_BATCH_SIZE + 1
        })
    );

    // Batch subscription: invalid topic.
    let request = Request {
        id,
        jsonrpc: jsonrpc.clone(),
        params: Params::BatchSubscribe(BatchSubscribe {
            topics: vec![Topic::from(
                "c4163cf65859106b3f5435fc296e7765411178ed452d1c30337a6230138c98401",
            )],
        }),
    };
    assert_eq!(
        request.validate(),
        Err(ValidationError::TopicDecoding(DecodingError::Length))
    );

    // Batch unsubscription: valid.
    let request = Request {
        id,
        jsonrpc: jsonrpc.clone(),
        params: Params::BatchUnsubscribe(BatchUnsubscribe {
            subscriptions: vec![Unsubscribe {
                topic,
                subscription_id: subscription_id.clone(),
            }],
        }),
    };
    assert_eq!(request.validate(), Ok(()));

    // Batch unsubscription: empty list.
    let request = Request {
        id,
        jsonrpc: jsonrpc.clone(),
        params: Params::BatchUnsubscribe(BatchUnsubscribe {
            subscriptions: vec![],
        }),
    };
    assert_eq!(request.validate(), Err(ValidationError::BatchEmpty));

    // Batch unsubscription: too many items.
    let subscriptions = (0..MAX_SUBSCRIPTION_BATCH_SIZE + 1)
        .map(|_| Unsubscribe {
            topic: Topic::generate(),
            subscription_id: SubscriptionId::generate(),
        })
        .collect();
    let request = Request {
        id,
        jsonrpc: jsonrpc.clone(),
        params: Params::BatchUnsubscribe(BatchUnsubscribe { subscriptions }),
    };
    assert_eq!(
        request.validate(),
        Err(ValidationError::BatchLimitExceeded {
            limit: MAX_SUBSCRIPTION_BATCH_SIZE,
            actual: MAX_SUBSCRIPTION_BATCH_SIZE + 1
        })
    );

    // Batch unsubscription: invalid topic.
    let request = Request {
        id,
        jsonrpc: jsonrpc.clone(),
        params: Params::BatchUnsubscribe(BatchUnsubscribe {
            subscriptions: vec![Unsubscribe {
                topic: Topic::from(
                    "c4163cf65859106b3f5435fc296e7765411178ed452d1c30337a6230138c98401",
                ),
                subscription_id,
            }],
        }),
    };
    assert_eq!(
        request.validate(),
        Err(ValidationError::TopicDecoding(DecodingError::Length))
    );

    // Batch fetch: valid.
    let request = Request {
        id,
        jsonrpc: jsonrpc.clone(),
        params: Params::BatchFetchMessages(BatchFetchMessages {
            topics: vec![Topic::generate()],
        }),
    };
    assert_eq!(request.validate(), Ok(()));

    // Batch fetch: empty list.
    let request = Request {
        id,
        jsonrpc: jsonrpc.clone(),
        params: Params::BatchFetchMessages(BatchFetchMessages { topics: vec![] }),
    };
    assert_eq!(request.validate(), Err(ValidationError::BatchEmpty));

    // Batch fetch: too many items.
    let topics = (0..MAX_SUBSCRIPTION_BATCH_SIZE + 1)
        .map(|_| Topic::generate())
        .collect();
    let request = Request {
        id,
        jsonrpc: jsonrpc.clone(),
        params: Params::BatchFetchMessages(BatchFetchMessages { topics }),
    };
    assert_eq!(
        request.validate(),
        Err(ValidationError::BatchLimitExceeded {
            limit: MAX_SUBSCRIPTION_BATCH_SIZE,
            actual: MAX_SUBSCRIPTION_BATCH_SIZE + 1
        })
    );

    // Batch fetch: invalid topic.
    let request = Request {
        id,
        jsonrpc,
        params: Params::BatchFetchMessages(BatchFetchMessages {
            topics: vec![Topic::from(
                "c4163cf65859106b3f5435fc296e7765411178ed452d1c30337a6230138c98401",
            )],
        }),
    };
    assert_eq!(
        request.validate(),
        Err(ValidationError::TopicDecoding(DecodingError::Length))
    );
}
