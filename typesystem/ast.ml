type program = decl list [@@deriving yojson]

and tylit = TyName of string | TyInt [@@deriving yojson]

and decl =
  | Struct of {name: string; fields: struct_field list}
  | Enum of {name: string}
  | Trait of {name: string}
  | Impl of {trait: typepath; }
  | Function of {name: string; body: expr}
[@@deriving yojson]

and struct_field = {name: string; ty: tylit}

and lvalue = LName of string [@@deriving yojson]

and stmt = Let of {lvalue: lvalue; rvalue: expr} | Expr of expr
[@@deriving yojson]

and expr =
  | Unit
  | Int of int
  | Seq of stmt * expr
  | LValue of lvalue
  | StructCon of tylit
  | Call of expr * expr
[@@deriving yojson]

let rec seq (stmts : stmt list) (final : expr) : expr =
  match stmts with [] -> final | h :: t -> Seq (h, seq t final)
