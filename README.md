Reproduction of https://github.com/LukeMathWalker/wiremock-rs/issues/120

`cargo test` will sometimes fail

`cargo test -- --test-threads 1` will never fail

```
me@host wiremock-404s % cargo test
    Finished test [unoptimized + debuginfo] target(s) in 0.05s
     Running unittests src/main.rs (target/debug/deps/wiremock_404s-499d05d7d756f49b)

running 3 tests
test test::test_with_context1 ... ok
test test::test_with_context_special ... ok
test test::test_with_context2 ... FAILED

failures:

---- test::test_with_context2 stdout ----
thread 'test::test_with_context2' panicked at src/main.rs:50:9:
assertion `left == right` failed
  left: 404
 right: 200
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    test::test_with_context2

test result: FAILED. 2 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

error: test failed, to rerun pass `--bin wiremock-404s`
me@host wiremock-404s % cargo test
    Finished test [unoptimized + debuginfo] target(s) in 0.05s
     Running unittests src/main.rs (target/debug/deps/wiremock_404s-499d05d7d756f49b)

running 3 tests
test test::test_with_context_special ... ok
test test::test_with_context2 ... ok
test test::test_with_context1 ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```
