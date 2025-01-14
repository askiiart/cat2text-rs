#[test]
fn test_anybase_base4() {
    use crate::{anybase, base4};
    let text = "i love cats".to_string();

    let encoded = anybase::encode(text.clone(), 4, anybase::char_length(4));
    assert_eq!(base4::encode(text), encoded);

    let decoded = anybase::decode(encoded.clone(), 4, anybase::char_length(4));
    assert_eq!(base4::decode(encoded), decoded);
}
