use discretionary::make_optional;

#[make_optional]
struct Named {
    a: bool,
}

#[make_optional]
struct Unnamed(bool);

#[make_optional]
struct Unit;

#[make_optional]
struct WithGeneric<T> {
    a: T,
    b: i64,
}

#[test]
fn test_named() {
    let face = Named { a: Some(true) };
    assert!(face.a.unwrap());
}

#[test]
fn test_unnamed() {
    let face = Unnamed(Some(true));
    assert!(face.0.unwrap());
}

#[test]
fn test_unit() {
    let _unit = Unit;
}

#[test]
fn test_generic() {
    let face = WithGeneric {
        a: Some(84usize),
        b: Some(-5),
    };
    assert_eq!(face.a, Some(84));
    assert_eq!(face.b, Some(-5));
}
