mochi
=====

[![Build Status](https://ci.iptq.io/api/badges/mochi/compiler/status.svg)](https://ci.iptq.io/mochi/compiler)

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

- [x] Parsing
- [ ] Type-checking

Fixes:
- [ ] Use a less-fucked RegexSet implementation.

Contact
-------

Author: Michael Zhang

License: plz no copy
