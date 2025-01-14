#[test]
fn test_anybase_base4() {
    use crate::{anybase, base4};
    let text = "i love cats".to_string();

    let encoded = anybase::encode(text.clone(), 4, anybase::char_length(4));
    assert_eq!(base4::encode(text), encoded);

    let decoded = anybase::decode(encoded.clone(), 4, anybase::char_length(4));
    assert_eq!(base4::decode(encoded), decoded);
}

#[test]
fn test_anybase_encode() {
    use crate::anybase;
    let text = "i love cats".to_string();
    let base = 10;
    let char_length = anybase::char_length(base);

    assert_eq!("meow mewo; mrrp mreow mrrp nyaaaa~ mreow mreow meow nyaaaa~; meow mrow meow mrrp mreow meow mrrp mewo", anybase::encode(text, base, char_length));
}

#[test]
fn test_anybase_decode() {
    use crate::anybase;
    let text = "meow mewo; mrrp mreow mrrp nyaaaa~ mreow mreow meow nyaaaa~; meow mrow meow mrrp mreow meow mrrp mewo".to_string();
    let base = 10;
    let char_length = anybase::char_length(base);
    assert_eq!("i love cats", anybase::decode(text, base, char_length));
}
