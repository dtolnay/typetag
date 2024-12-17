#![allow(clippy::extra_unused_type_parameters)]

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct A {
    a: u8,
}

#[derive(Serialize, Deserialize)]
struct B {
    b: u8,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
struct C {
    c: u8,
}

mod externally_tagged {
    use super::{A, B};

    #[typetag::serde]
    trait Trait {
        fn assert_a_is_11(&self);
        fn assert_b_is_11(&self);
    }

    #[typetag::serde]
    impl Trait for A {
        fn assert_a_is_11(&self) {
            assert_eq!(self.a, 11);
        }
        fn assert_b_is_11(&self) {
            panic!("is not B!");
        }
    }

    #[typetag::serde]
    impl Trait for B {
        fn assert_a_is_11(&self) {
            panic!("is not A!");
        }
        fn assert_b_is_11(&self) {
            assert_eq!(self.b, 11);
        }
    }

    #[test]
    fn test_json_serialize() {
        let trait_object = &A { a: 11 } as &dyn Trait;
        let json = serde_json::to_string(trait_object).unwrap();
        let expected = r#"{"A":{"a":11}}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_json_deserialize() {
        let json = r#"{"B":{"b":11}}"#;
        let trait_object: Box<dyn Trait> = serde_json::from_str(json).unwrap();
        trait_object.assert_b_is_11();
    }

    #[test]
    fn test_postcard_round_trip() {
        let trait_object = &A { a: 11 } as &dyn Trait;
        let bytes = postcard::to_stdvec(trait_object).unwrap();
        let trait_object: Box<dyn Trait> = postcard::from_bytes(&bytes).unwrap();
        trait_object.assert_a_is_11();
    }
}

mod internally_tagged {
    use super::{A, B, C};

    #[typetag::serde(tag = "type")]
    trait Trait {
        fn assert_a_is_11(&self);
        fn assert_b_is_11(&self);
    }

    #[typetag::serde]
    impl Trait for A {
        fn assert_a_is_11(&self) {
            assert_eq!(self.a, 11);
        }
        fn assert_b_is_11(&self) {
            panic!("is not B!");
        }
    }

    #[typetag::serde]
    impl Trait for B {
        fn assert_a_is_11(&self) {
            panic!("is not A!");
        }
        fn assert_b_is_11(&self) {
            assert_eq!(self.b, 11);
        }
    }

    #[typetag::serde]
    impl Trait for C {
        fn assert_a_is_11(&self) {
            panic!("is not A!");
        }
        fn assert_b_is_11(&self) {
            panic!("is not B!");
        }
    }

    #[test]
    fn test_json_serialize() {
        let trait_object = &A { a: 11 } as &dyn Trait;
        let json = serde_json::to_string(trait_object).unwrap();
        let expected = r#"{"type":"A","a":11}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_json_serialize_with_serde_tag() {
        let trait_object = &C { c: 11 } as &dyn Trait;
        let json = serde_json::to_string(trait_object).unwrap();
        let expected = r#"{"type":"C","c":11}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_json_deserialize() {
        let json = r#"{"type":"B","b":11}"#;
        let trait_object: Box<dyn Trait> = serde_json::from_str(json).unwrap();
        trait_object.assert_b_is_11();
    }

    #[test]
    fn test_postcard_round_trip() {
        let trait_object = &A { a: 11 } as &dyn Trait;
        let bytes = postcard::to_stdvec(trait_object).unwrap();
        let trait_object: Box<dyn Trait> = postcard::from_bytes(&bytes).unwrap();
        trait_object.assert_a_is_11();
    }
}

mod adjacently_tagged {
    use super::{A, B};

    #[typetag::serde(tag = "type", content = "content")]
    trait Trait {
        fn assert_a_is_11(&self);
        fn assert_b_is_11(&self);
    }

    #[typetag::serde]
    impl Trait for A {
        fn assert_a_is_11(&self) {
            assert_eq!(self.a, 11);
        }
        fn assert_b_is_11(&self) {
            panic!("is not B!");
        }
    }

    #[typetag::serde]
    impl Trait for B {
        fn assert_a_is_11(&self) {
            panic!("is not A!");
        }
        fn assert_b_is_11(&self) {
            assert_eq!(self.b, 11);
        }
    }

    #[test]
    fn test_json_serialize() {
        let trait_object = &A { a: 11 } as &dyn Trait;
        let json = serde_json::to_string(trait_object).unwrap();
        let expected = r#"{"type":"A","content":{"a":11}}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_json_deserialize() {
        let json = r#"{"type":"B","content":{"b":11}}"#;
        let trait_object: Box<dyn Trait> = serde_json::from_str(json).unwrap();
        trait_object.assert_b_is_11();

        let json = r#"{"type":"B","content":{"b":11},"unknown":null}"#;
        let trait_object: Box<dyn Trait> = serde_json::from_str(json).unwrap();
        trait_object.assert_b_is_11();
    }

    #[test]
    fn test_postcard_round_trip() {
        let trait_object = &A { a: 11 } as &dyn Trait;
        let bytes = postcard::to_stdvec(trait_object).unwrap();
        let trait_object: Box<dyn Trait> = postcard::from_bytes(&bytes).unwrap();
        trait_object.assert_a_is_11();
    }
}

mod other_types {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    struct A {}

    #[derive(Serialize, Deserialize)]
    struct B;

    #[derive(Serialize, Deserialize)]
    enum C {
        Foo,
    }

    #[typetag::serde]
    trait Trait {
        fn assert_is_a(&self);
        fn assert_is_b(&self);
        fn assert_is_c(&self);
    }

    #[typetag::serde]
    impl Trait for A {
        fn assert_is_a(&self) {}
        fn assert_is_b(&self) {
            panic!("is A");
        }
        fn assert_is_c(&self) {
            panic!("is A");
        }
    }

    #[typetag::serde]
    impl Trait for B {
        fn assert_is_a(&self) {
            panic!("is B");
        }
        fn assert_is_b(&self) {}
        fn assert_is_c(&self) {
            panic!("is B");
        }
    }

    #[typetag::serde]
    impl Trait for C {
        fn assert_is_a(&self) {
            panic!("is C");
        }
        fn assert_is_b(&self) {
            panic!("is C");
        }
        fn assert_is_c(&self) {}
    }

    #[test]
    fn test_json_round_trip() {
        let trait_object = &A {} as &dyn Trait;
        let json = serde_json::to_string(trait_object).unwrap();
        let expected = r#"{"A":{}}"#;
        assert_eq!(json, expected);
        let round_trip_object: Box<dyn Trait> = serde_json::from_str(&json).unwrap();
        round_trip_object.assert_is_a();

        let trait_object = &B as &dyn Trait;
        let json = serde_json::to_string(trait_object).unwrap();
        let expected = r#"{"B":null}"#;
        assert_eq!(json, expected);
        let round_trip_object: Box<dyn Trait> = serde_json::from_str(&json).unwrap();
        round_trip_object.assert_is_b();

        let trait_object = &C::Foo as &dyn Trait;
        let json = serde_json::to_string(trait_object).unwrap();
        let expected = r#"{"C":"Foo"}"#;
        assert_eq!(json, expected);
        let round_trip_object: Box<dyn Trait> = serde_json::from_str(&json).unwrap();
        round_trip_object.assert_is_c();
    }

    #[test]
    fn test_postcard_round_trip() {
        let trait_object = &A {} as &dyn Trait;
        let bytes = postcard::to_stdvec(trait_object).unwrap();
        let trait_object: Box<dyn Trait> = postcard::from_bytes(&bytes).unwrap();
        trait_object.assert_is_a();

        let trait_object = &B as &dyn Trait;
        let bytes = postcard::to_stdvec(trait_object).unwrap();
        let trait_object: Box<dyn Trait> = postcard::from_bytes(&bytes).unwrap();
        trait_object.assert_is_b();

        let trait_object = &C::Foo as &dyn Trait;
        let bytes = postcard::to_stdvec(trait_object).unwrap();
        let trait_object: Box<dyn Trait> = postcard::from_bytes(&bytes).unwrap();
        trait_object.assert_is_c();
    }
}

mod internal_with_default {
    use super::{A, B};

    #[typetag::serde(tag = "type", default_variant = "A")]
    trait Trait {
        fn assert_a_is_11(&self);
        fn assert_b_is_11(&self);
    }

    #[typetag::serde]
    impl Trait for A {
        fn assert_a_is_11(&self) {
            assert_eq!(self.a, 11);
        }
        fn assert_b_is_11(&self) {
            panic!("is not B!");
        }
    }

    #[typetag::serde]
    impl Trait for B {
        fn assert_a_is_11(&self) {
            panic!("is not A!");
        }
        fn assert_b_is_11(&self) {
            assert_eq!(self.b, 11);
        }
    }

    #[test]
    fn test_json_deserialize_default_variant() {
        let json = r#"{"a":11}"#;
        let trait_object: Box<dyn Trait> = serde_json::from_str(json).unwrap();
        trait_object.assert_a_is_11();
    }

    #[test]
    fn test_json_deserialize_named_variant() {
        let json = r#"{"type":"B","b":11}"#;
        let trait_object: Box<dyn Trait> = serde_json::from_str(json).unwrap();
        trait_object.assert_b_is_11();
    }
}

mod adjacent_with_default {
    use super::{A, B};

    #[typetag::serde(tag = "type", content = "content", default_variant = "A")]
    trait Trait {
        fn assert_a_is_11(&self);
        fn assert_b_is_11(&self);
    }

    #[typetag::serde]
    impl Trait for A {
        fn assert_a_is_11(&self) {
            assert_eq!(self.a, 11);
        }
        fn assert_b_is_11(&self) {
            panic!("is not B!");
        }
    }

    #[typetag::serde]
    impl Trait for B {
        fn assert_a_is_11(&self) {
            panic!("is not A!");
        }
        fn assert_b_is_11(&self) {
            assert_eq!(self.b, 11);
        }
    }

    #[test]
    fn test_json_deserialize_default_variant() {
        let json = r#"{"content":{"a":11}}"#;
        let trait_object: Box<dyn Trait> = serde_json::from_str(json).unwrap();
        trait_object.assert_a_is_11();
    }

    #[test]
    fn test_json_deserialize_named_variant() {
        let json = r#"{"type":"B","content":{"b":11}}"#;
        let trait_object: Box<dyn Trait> = serde_json::from_str(json).unwrap();
        trait_object.assert_b_is_11();
    }
}

mod adjacent_deny_unknown {
    use super::{A, B};

    #[typetag::serde(tag = "type", content = "content", deny_unknown_fields)]
    trait Trait {}

    #[typetag::serde]
    impl Trait for A {}

    #[typetag::serde]
    impl Trait for B {}

    #[test]
    fn test_json_deserialize_deny_unknown() {
        let json = r#"{"type":"B","content":{"b":11},"unknown":null}"#;
        match serde_json::from_str::<Box<dyn Trait>>(json) {
            Ok(_) => panic!("unexpectedly deserialized successfully despite unknown field"),
            Err(err) => {
                let expected =
                    "unknown field `unknown`, expected `type` or `content` at line 1 column 40";
                assert_eq!(err.to_string(), expected);
            }
        }
    }
}

mod marker_traits {
    use serde::de::DeserializeOwned;
    use serde::Serialize;

    #[typetag::serde]
    trait Neither {}

    #[typetag::serde]
    trait Sendable: Send {}

    #[typetag::serde]
    trait Syncable: Sync {}

    #[typetag::serde]
    trait Both: Send + Sync {}

    fn assert_serialize<T>()
    where
        T: ?Sized + Serialize,
    {
    }

    fn assert_deserialize<T>()
    where
        T: ?Sized,
        Box<T>: DeserializeOwned,
    {
    }

    #[test]
    fn test_serialize() {
        assert_serialize::<dyn Neither>();
        assert_serialize::<dyn Neither + Send>();
        assert_serialize::<dyn Neither + Sync>();
        assert_serialize::<dyn Neither + Send + Sync>();

        assert_serialize::<dyn Sendable>();
        assert_serialize::<dyn Sendable + Send>();
        assert_serialize::<dyn Sendable + Sync>();
        assert_serialize::<dyn Sendable + Send + Sync>();

        assert_serialize::<dyn Syncable>();
        assert_serialize::<dyn Syncable + Send>();
        assert_serialize::<dyn Syncable + Sync>();
        assert_serialize::<dyn Syncable + Send + Sync>();

        assert_serialize::<dyn Both>();
        assert_serialize::<dyn Both + Send>();
        assert_serialize::<dyn Both + Sync>();
        assert_serialize::<dyn Both + Send + Sync>();
    }

    #[test]
    fn test_deserialize() {
        assert_deserialize::<dyn Neither>();

        assert_deserialize::<dyn Sendable>();
        assert_deserialize::<dyn Sendable + Send>();

        assert_deserialize::<dyn Syncable>();
        assert_deserialize::<dyn Syncable + Sync>();

        assert_deserialize::<dyn Both>();
        assert_deserialize::<dyn Both + Send>();
        assert_deserialize::<dyn Both + Sync>();
        assert_deserialize::<dyn Both + Send + Sync>();
    }
}

mod generic {
    #[typetag::serialize]
    trait Generic<T> {}
}

#[rustversion::since(1.74)]
mod assoc_type {
    #[typetag::serde]
    trait Trait {
        type AssocType
        where
            Self: Sized;
    }
}

mod macro_expanded {
    use super::A;

    #[typetag::serde]
    trait Trait {}

    macro_rules! impl_trait {
        ($ty:ty) => {
            #[typetag::serde]
            impl Trait for $ty {}
        };
    }

    impl_trait!(A);
}

// https://github.com/dtolnay/typetag/issues/28
mod trait_hierarchy {
    use serde::{Deserialize, Serialize};

    #[typetag::serde]
    pub trait Base {}

    #[derive(Serialize, Deserialize)]
    struct SomeBase;

    #[typetag::serde]
    impl Base for SomeBase {}

    #[typetag::serde]
    pub trait Derived: Base {}

    #[derive(Serialize, Deserialize)]
    struct SomeDerived;

    #[typetag::serde]
    impl Base for SomeDerived {}

    #[typetag::serde]
    impl Derived for SomeDerived {}
}

mod tag_mismatch {
    use serde::Serialize;

    #[typetag::serialize(tag = "type")]
    trait Trait {}

    #[derive(Serialize)]
    struct Tagged<T> {
        #[serde(rename = "type")]
        t: T,
    }

    #[typetag::serialize]
    impl<T: Serialize> Trait for Tagged<T> {}

    #[test]
    fn test_json_serialize() {
        let trait_object = &Tagged { t: "Tagged" } as &dyn Trait;
        let json = serde_json::to_string(trait_object).unwrap();
        let expected = r#"{"type":"Tagged"}"#;
        assert_eq!(json, expected);

        let trait_object = &Tagged { t: "Mismatch" } as &dyn Trait;
        let err = serde_json::to_string(trait_object).unwrap_err();
        let expected = r#"mismatched value for tag "type": "Tagged" vs "Mismatch""#;
        assert_eq!(err.to_string(), expected);

        let trait_object = &Tagged { t: false } as &dyn Trait;
        let err = serde_json::to_string(trait_object).unwrap_err();
        let expected = r#"mismatched value for tag "type": "Tagged" vs non-string"#;
        assert_eq!(err.to_string(), expected);
    }
}

mod async_traits {
    use async_trait::async_trait;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct InOrder;

    #[derive(Serialize, Deserialize)]
    pub struct OutOfOrder;

    #[typetag::serde]
    #[async_trait]
    trait TypetagAsync {
        async fn f(&self) {}
    }

    #[typetag::serde]
    #[async_trait]
    impl TypetagAsync for InOrder {
        async fn f(&self) {}
    }

    #[async_trait]
    #[typetag::serde]
    impl TypetagAsync for OutOfOrder {
        async fn f(&self) {}
    }

    #[async_trait]
    #[typetag::serde]
    trait AsyncTypetag {
        async fn f(&self) {}
    }

    #[async_trait]
    #[typetag::serde]
    impl AsyncTypetag for InOrder {
        async fn f(&self) {}
    }

    #[typetag::serde]
    #[async_trait]
    impl AsyncTypetag for OutOfOrder {
        async fn f(&self) {}
    }
}
