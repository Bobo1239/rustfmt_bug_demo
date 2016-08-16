    $ cargo run
        Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
         Running `target\debug\rustfmt_bug_demo.exe`


    $ cargo fmt -- --write-mode=diff
    Using rustfmt config file \\?\E:\development\Rust\bms\rustfmt.toml for E:\development\Rust\bms\bms_parser\bug\src\main.rs
    Diff in E:\development\Rust\bms\bms_parser\bug\src\main.rs at line 11:
     ⏎
     fn ugly_function() {⏎
         // ugly code which gets formatted as it's before the macro call⏎
    -    println!(    "AS"    );⏎
    +    println!("AS");⏎
     }⏎
     ⏎
     fn macro_call() -> bool {⏎


    $ cargo fmt -- --write-mode=diff
    Using rustfmt config file \\?\E:\development\Rust\bms\rustfmt.toml for E:\development\Rust\bms\bms_parser\bug\src\main.rs
    Diff in E:\development\Rust\bms\bms_parser\bug\src\module.rs at line 6:
     ⏎
     pub fn function() {⏎
         // gets formatted⏎
    -    assert!(   mac!(<)   ,  true);⏎
    +    assert!(mac!(<), true);⏎
         // doesn't get formatted⏎
         assert!(   mac!(<)   ,  true);⏎
     }⏎
