#[macro_export]
macro_rules! avec {
    () => {
        Vec::new()
    };
    ($element:expr) => {{
        let mut vs = Vec::new();
        vs.push($element);
        vs
    }};
}
#[test]
fn single() {
    let x: Vec<u32> = avec![42];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 1);
    assert_eq!(x[0], 42);
}

#[test]
fn empty_vec() {
    let x: Vec<u32> = avec![];
    assert!(x.is_empty());
}
