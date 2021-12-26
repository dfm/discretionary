use discretionary::make_optional;

#[test]
fn test_named() {
    #[make_optional]
    struct Named {
        a: bool,
    }

    let face = Named { a: Some(true) };
    assert!(face.a.unwrap());
}

#[test]
fn test_unnamed() {
    #[make_optional]
    struct Unnamed(bool);

    let face = Unnamed(Some(true));
    assert!(face.0.unwrap());
}

#[test]
fn test_unit() {
    #[make_optional]
    struct Unit;

    let _unit = Unit;
}

#[test]
fn test_generic() {
    #[make_optional]
    struct WithGeneric<T> {
        a: T,
        b: i64,
    }

    let face = WithGeneric {
        a: Some(84usize),
        b: Some(-5),
    };
    assert_eq!(face.a, Some(84));
    assert_eq!(face.b, Some(-5));
}
