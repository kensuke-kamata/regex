# regex

A simple regex engine.

## Example

```rust
use regex;
let expr = "abc|(de|cd)+";
let line = "decddede";
// Checks if the regex expression matches the line by DFS.
regex::is_match(expr, line, true);
```

## Functions

- `is_match`: Checks if the regex expression matches the line.
- `print`: Parses the regex expression and generates the code. Prints the AST and the generated code to stdout.

## Supported Expressions

- Literals: `a`, `b`, `c`, etc.
- Quantifiers: `+`, `*`, `?`
- Grouping: `(...)`
- Alternation: `|`

TODO
- `.`, `^`, `$`, etc.
