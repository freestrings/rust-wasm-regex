#rust-wasm-regex

Rust [**regex**](https://crates.io/crates/regex)도 WebAssembly에서 동작된다.

```javascript

...
let regStrPtr = newString(module, regexToStr(/[\-\[\]{}()*+?.,\\\^$|#\s]/));
let regPtr = module.create_regexp(regStrPtr);

// 정규식 문자열로 escape
module.escape_as_reg(regPtr, newString(module, "a-[]{}()*+?.,\\^$|#\\s한b"));
...
// 정규식 객체로 escape
module.escape_as_regstr(regStrPtr, newString(module, "a-[]{}()*+?.,\\^$|#\\s한b"));
...
// 결과
// => "a\\-\\[\\]\\{\\}\\(\\)\\*\\+\\?\\.\\,\\\\\\^\\$\\|\\#\\\\s한b"
//

```

```rust
...
#[test]
fn escape() {
    let reg = super::Regex::new(r"[\-\[\]{}()*+?.,\\\^$|#\s]").unwrap();

    assert_eq!(
        super::escape(&reg, "a-[]{}()*+?.,\\^$|#\\s한b"), 
        "a\\-\\[\\]\\{\\}\\(\\)\\*\\+\\?\\.\\,\\\\\\^\\$\\|\\#\\\\s한b"
    );
}
...

```