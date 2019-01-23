use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct A {
    a: u8,
}

#[derive(Serialize, Deserialize)]
struct B {
    b: u8,
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
        let trait_object = &A { a: 11 } as &Trait;
        let json = serde_json::to_string(trait_object).unwrap();
        let expected = r#"{"A":{"a":11}}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_json_deserialize() {
        let json = r#"{"B":{"b":11}}"#;
        let trait_object: Box<Trait> = serde_json::from_str(json).unwrap();
        trait_object.assert_b_is_11();
    }

    #[test]
    fn test_bincode_round_trip() {
        let trait_object = &A { a: 11 } as &Trait;
        let bytes = bincode::serialize(trait_object).unwrap();
        let trait_object: Box<Trait> = bincode::deserialize(&bytes).unwrap();
        trait_object.assert_a_is_11();
    }
}

mod internally_tagged {
    use super::{A, B};

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

    #[test]
    fn test_json_serialize() {
        let trait_object = &A { a: 11 } as &Trait;
        let json = serde_json::to_string(trait_object).unwrap();
        let expected = r#"{"type":"A","a":11}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_json_deserialize() {
        let json = r#"{"type":"B","b":11}"#;
        let trait_object: Box<Trait> = serde_json::from_str(json).unwrap();
        trait_object.assert_b_is_11();
    }

    #[test]
    fn test_bincode_round_trip() {
        let trait_object = &A { a: 11 } as &Trait;
        let bytes = bincode::serialize(trait_object).unwrap();
        let trait_object: Box<Trait> = bincode::deserialize(&bytes).unwrap();
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
        let trait_object = &A { a: 11 } as &Trait;
        let json = serde_json::to_string(trait_object).unwrap();
        let expected = r#"{"type":"A","content":{"a":11}}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_json_deserialize() {
        let json = r#"{"type":"B","content":{"b":11}}"#;
        let trait_object: Box<Trait> = serde_json::from_str(json).unwrap();
        trait_object.assert_b_is_11();
    }

    #[test]
    fn test_bincode_round_trip() {
        let trait_object = &A { a: 11 } as &Trait;
        let bytes = bincode::serialize(trait_object).unwrap();
        let trait_object: Box<Trait> = bincode::deserialize(&bytes).unwrap();
        trait_object.assert_a_is_11();
    }
}
