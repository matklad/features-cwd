```
~/tmp master*
λ pushd bar; cargo +stable run --package foo; popd
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `/home/matklad/tmp/target/debug/foo`
xyz disabled

~/tmp master*
λ pushd baz; cargo +stable run --package foo; popd
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `/home/matklad/tmp/target/debug/foo`
xyz enabled

~/tmp master*
λ pushd bar; cargo +nightly run --package foo; popd
    Finished dev [unoptimized + debuginfo] target(s) in 0.00 secs
     Running `/home/matklad/tmp/target/debug/foo`
xyz disabled

~/tmp master*
λ pushd baz; cargo +nightly run --package foo; popd
    Finished dev [unoptimized + debuginfo] target(s) in 0.00 secs
     Running `/home/matklad/tmp/target/debug/foo`
xyz disabled
```     
