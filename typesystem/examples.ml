open Ast

let programHello =
  [Function {name= "main"; body= seq [ (* Print "Hello, world!" *) ] Unit}]

let programIterator =
  [ Struct
      { name= "Fibonacci"
      ; fields= [{name= "curr"; ty= TyInt}; {name= "next"; ty= TyInt}] }
  ; Function
      { name= "main"
      ; body=
          seq
            [ Let {lvalue= LName "fib"; rvalue= StructCon (TyName "Fibonacci")}
            ; Expr (Call (LValue (LName "fib"), Unit)) ]
            Unit } ]

let programs = [programHello; programIterator]

let _ =
  List.iter
    (fun program ->
      program |> Typed_ast.convert_program |> Typed_ast.program_to_yojson
      |> Yojson.Safe.to_string |> print_endline )
    programs
