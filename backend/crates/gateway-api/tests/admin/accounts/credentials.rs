use gateway_api::admin::accounts::UpdateAccountRequest;
use serde_json::json;

#[test]
fn state_length_update_distinguishes_omission_null_and_value() {
    let mut body = json!({
        "accountId": "acct_state_length", "enabled": true,
        "concurrencyLimit": null, "weight": 1, "groupIds": []
    });
    let missing: UpdateAccountRequest = serde_json::from_value(body.clone()).unwrap();
    assert_eq!(missing.session_keepalive_expected_lengths, None);
    body["sessionKeepaliveExpectedLengths"] = json!(null);
    let cleared: UpdateAccountRequest = serde_json::from_value(body.clone()).unwrap();
    assert_eq!(cleared.session_keepalive_expected_lengths, Some(None));
    for lengths in [
        vec![1],
        vec![99],
        vec![292, 312],
        vec![332, 356],
        vec![2001],
        vec![u32::MAX],
    ] {
        body["sessionKeepaliveExpectedLengths"] = json!(lengths);
        let request: UpdateAccountRequest = serde_json::from_value(body.clone()).unwrap();
        assert_eq!(
            request.session_keepalive_expected_lengths,
            Some(Some(lengths.into_iter().map(Into::into).collect()))
        );
        assert!(request.validate().is_ok());
    }
    for lengths in [vec![], vec![0], vec![292, 292], vec![292; 33]] {
        body["sessionKeepaliveExpectedLengths"] = json!(lengths);
        let request: UpdateAccountRequest = serde_json::from_value(body.clone()).unwrap();
        assert!(request.validate().is_err());
    }
    for value in [json!([-1]), json!([292.5]), json!(["332"])] {
        body["sessionKeepaliveExpectedLengths"] = value;
        assert!(serde_json::from_value::<UpdateAccountRequest>(body.clone()).is_err());
    }
    body["sessionKeepaliveExpectedLengths"] = json!([200, 300, {"min":400,"max":500}]);
    let request: UpdateAccountRequest = serde_json::from_value(body.clone()).unwrap();
    assert!(request.validate().is_ok());
    let rules = request.session_keepalive_expected_lengths.unwrap().unwrap();
    for (length, expected) in [
        (199, false),
        (200, true),
        (300, true),
        (399, false),
        (400, true),
        (450, true),
        (500, true),
        (501, false),
    ] {
        assert_eq!(
            rules.iter().any(|rule| rule.contains(length)),
            expected,
            "{length}"
        );
    }
    for value in [
        json!([{ "min":500, "max":400 }]),
        json!([{ "min":0, "max":400 }]),
        json!([200, {"min":200,"max":200}]),
    ] {
        body["sessionKeepaliveExpectedLengths"] = value;
        let request: UpdateAccountRequest = serde_json::from_value(body.clone()).unwrap();
        assert!(request.validate().is_err());
    }
    for value in [
        json!([{ "min":400 }]),
        json!([{ "min":400, "max":500, "extra":1 }]),
        json!([4294967296_u64]),
    ] {
        body["sessionKeepaliveExpectedLengths"] = value;
        assert!(serde_json::from_value::<UpdateAccountRequest>(body.clone()).is_err());
    }
}
