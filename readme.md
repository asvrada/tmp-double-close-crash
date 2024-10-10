# Double closing crash reproduce demo

# What's the issue

Calling `NCryptFreeObject` twice on the same `NCRYPT_PROV_HANDLE` would some times crash the program.

For reproducing code, See [test_double_close_provider](src/lib.rs)

# How to re-produce

Simply run `cargo test` multiple times.

Below is a log on my machine, I had to re-run it multiple times before it crashed.
```
PS C:\code\test-double-close> cargo test
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.04s     Running unittests src\lib.rs (target\debug\deps\test_doubl
e_close-bbc83a4dc92d54bf.exe)

running 1 test
error: test failed, to rerun pass `--lib`

Caused by:
  process didn't exit successfully: `C:\code\test-double-close\target\debug\deps\test_double_close-bbc83a4dc92d54bf.exe` (exit code: 0xc0000374, STATUS_HEAP_CORRUPTION)
note: test exited abnormally; to see the full output pass --nocapture to the harness.
PS C:\code\test-double-close>
```

After the first crash, following `cargo test` would have a much higher chance of running into this issue
```
PS C:\code\test-double-close> cargo test -- --nocapture
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running unittests src\lib.rs (target\debug\deps\test_double_close-bbc83a4dc92d54bf.exe)

running 1 test
error: test failed, to rerun pass `--lib`

Caused by:
  process didn't exit successfully: `C:\code\test-double-close\target\debug\deps\test_double_close-bbc83a4dc92d54bf.exe --nocapture` (exit code: 0xc0000374, STATUS_HEAP_CORRUPTION)
```
