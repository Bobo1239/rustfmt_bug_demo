// if main.rs is formatted first, module.rs doesn't get formatted
// if module.rs is formatted first, main.rs doesn't get formatted

mod module;

// macro which uses <...> and breaks rustfmt later on
// real world usage: e.g. http://rust.unhandledexpression.com/nom/macro.named!.html
// actually < alone suffices (e.g. (<Type) => {...})
macro_rules! macr {
    (<Type>) => {
        true
    };
}

fn ugly_function() {
    // ugly code which gets formatted as it's before the macro call
    println!(    "AS"    );
}

fn macro_call() -> bool {
    // calling the macro; everything beyond this line doesn't get formatted
    macr!(<Type>)
}

fn main() {
    // ugly code which doesn't get formatted
    assert_eq!(   macro_call()  ,   macro_call()  );
}
