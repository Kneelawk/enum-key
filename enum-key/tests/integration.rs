use enum_key::{EnumKey, KeyableEnum};

#[derive(KeyableEnum)]
#[keyable_enum(key_enum = AKey)]
pub enum MyEnum {
    A(u8),
    B { inside: u32 },
    C,
}

#[test]
fn test_values() {
    assert_eq!(
        <MyEnum as KeyableEnum>::Key::VALUES,
        &[AKey::A, AKey::B, AKey::C],
        "VALUES should contain all key variants"
    )
}

#[test]
fn test_variant() {
    assert_eq!(
        MyEnum::A(0).get_enum_key(),
        AKey::A,
        "Variant A should have the key A"
    )
}
