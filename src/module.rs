macro_rules! mac {
    (<) => {
        true
    };
}

pub fn function() {
    // gets formatted
    assert!(   mac!(<)   ,  true);
    // doesn't get formatted
    assert!(   mac!(<)   ,  true);
}
