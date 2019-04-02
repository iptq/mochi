mochi
=====

[![Build Status](https://ci.iptq.io/api/badges/mochi/mochi/status.svg)](https://ci.iptq.io/mochi/mochi)

Example
-------

```
use std.io.(Scanner, stdin)

func each (n: int) -> string =
  match (i % 3, i % 5):
    (0, 0) => "FizzBuzz"
    (0, _) => "Fizz"
    (_, 0) => "Buzz"
    _ => n

func main =
  let scanner = Scanner.new stdin
  let n = scanner.nextInt ()

  for i in 0..n:
    print (each i)
```

Todos
-----

- [x] Syntax
  - [x] Custom Lexer
  - [x] Parsing
- [ ] Semantic Checking
  - [x] Type Checking
    - [x] Type Inference
    - [ ] Typeclasses
    - [ ] First-class Functions
  - [x] Good Error Reporting
- [ ] Modules
- [ ] IR / Codegen
  - [ ] Closure conversion
- [ ] Garbage Collector
- [ ] Standard Library
  - [ ]
- [x] Interpreter
- [ ] Documentation (lol)

Fixes:
- [ ] Use a less-fucked RegexSet implementation.

Contact
-------

Author: Michael Zhang

License: plz no copy
