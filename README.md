Implementation of the Unicode Bidirectional Algorithm (UBA).

Reference: [http://www.unicode.org/reports/tr9/](http://www.unicode.org/reports/tr9/).

Converts logical strings to their equivalent visual representation. Persian, Hebrew and Arabic languages (and any other RTL language) are supported.

```rust
use bidi:*;

let text: &str = "مهدی";
let mut text_u16 = text.encode_utf16().collect::<Vec<u16>>();

perform_shaping(&mut text_u16);

let result = String::from_utf16(text_u16.as_slice()).unwrap();
assert_eq!(result, "ﻣﻬﺪﯼ");
```
