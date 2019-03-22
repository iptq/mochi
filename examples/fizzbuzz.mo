use std.io.(Scanner, stdin)

func each (n: int):
  match (i % 3, i % 5):
    0, 0 => "FizzBuzz"
    0, _ => "Fizz"
    _, 0 => "Buzz"
    _ => n.toString ()

func main:
  let scanner = Scanner.new stdin
  let n = scanner.nextInt ()

  for i in 0..n:
    print (each i)
