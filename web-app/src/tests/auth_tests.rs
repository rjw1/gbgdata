use crate::auth::*;

#[test]
fn test_password_roundtrip() {
    let password = "correct-horse-battery-staple";
    let hash = hash_password(password).unwrap();
    assert!(verify_password(password, &hash));
    assert!(!verify_password("wrong-password", &hash));
}

#[test]
fn test_invalid_hash_format() {
    assert!(!verify_password("any", "not-a-valid-hash"));
}
