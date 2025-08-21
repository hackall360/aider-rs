use aider_llm::sendchat::{ensure_alternating_roles, sanity_check_messages, Message};

#[test]
fn sanity_check_valid() {
    let msgs = vec![
        Message {
            role: "user".into(),
            content: "hi".into(),
        },
        Message {
            role: "assistant".into(),
            content: "hello".into(),
        },
        Message {
            role: "user".into(),
            content: "again".into(),
        },
    ];
    assert!(sanity_check_messages(&msgs).is_ok());
}

#[test]
fn sanity_check_invalid() {
    let msgs = vec![
        Message {
            role: "user".into(),
            content: "hi".into(),
        },
        Message {
            role: "user".into(),
            content: "again".into(),
        },
    ];
    assert!(sanity_check_messages(&msgs).is_err());
}

#[test]
fn ensure_alternating_inserts() {
    let msgs = vec![
        Message {
            role: "user".into(),
            content: "1".into(),
        },
        Message {
            role: "user".into(),
            content: "2".into(),
        },
    ];
    let fixed = ensure_alternating_roles(&msgs);
    assert_eq!(fixed.len(), 3);
    assert_eq!(fixed[1].role, "assistant");
    assert!(fixed[1].content.is_empty());
}
