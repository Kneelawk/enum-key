use enum_key::{EnumKey, KeyableEnum};

#[derive(KeyableEnum)]
pub enum MyEnum {
    A(u8),
    B { inside: u32 },
    C,
}

#[test]
fn test_values() {
    assert_eq!(
        <MyEnum as KeyableEnum>::Key::VALUES,
        &[MyEnumKey::A, MyEnumKey::B, MyEnumKey::C],
        "VALUES should contain all key variants"
    )
}

#[test]
fn test_variant() {
    assert_eq!(
        MyEnum::A(0).get_enum_key(),
        MyEnumKey::A,
        "Variant A should have the key A"
    )
}
