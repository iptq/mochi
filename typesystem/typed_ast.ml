type program = decl list [@@deriving yojson]

and ty = TyName of string | TyUnit | TyInt | TyVar of int [@@deriving yojson]

and signature =
  | StructSig of {fields: struct_field list}
  | EnumSig
  | ClassSig
  | FunctionSig
  | VarSig
[@@deriving yojson]

and decl =
  | Struct of {name: string; fields: struct_field list}
  | Enum of {name: string}
  | Class of {name: string}
  | Function of {name: string; body: expr; ty: ty}
[@@deriving yojson]

and struct_field = Ast.struct_field

and lvalue = Ast.lvalue

and stmt = Let of {lvalue: lvalue; rvalue: expr} | Expr of expr
[@@deriving yojson]

and expr =
  | Unit
  | Int of int
  | Seq of stmt * expr
  | LValue of lvalue * ty
  | StructCon of ty
  | Call of expr * expr * ty
[@@deriving yojson]

(* *)

module TypeEnv : sig
  type t

  val create : unit -> t

  val generate : unit -> ty

  val add : t -> string -> signature -> unit

  val lookup : t -> string -> signature option

  val all_keys : t -> (string, signature) Hashtbl.t

  val pprint : t -> string
end = struct
  type t = (string, signature) Hashtbl.t list

  let create () = [Hashtbl.create 10]

  let ty_counter = ref 0

  let generate () : ty =
    let n = !ty_counter in
    ty_counter := n + 1 ;
    TyVar n

  let add (env : t) (key : string) (value : signature) : unit =
    match env with
    | [] ->
        raise (Failure "Empty type environment")
    | h :: _ ->
        Hashtbl.add h key value

  let rec lookup (env : t) (key : string) : signature option =
    match env with
    | [] ->
        None
    | h :: t -> (
      match Hashtbl.find_opt h key with
      | Some v ->
          Some v
      | None ->
          lookup t key )

  let rec all_keys (env : t) : (string, signature) Hashtbl.t =
    match env with
    | [] ->
        Hashtbl.create 10
    | h :: t ->
        let up = all_keys t in
        Hashtbl.iter (fun k v -> Hashtbl.add up k v) h ;
        up

  let pprint (env : t) : string =
    let all = all_keys env in
    Hashtbl.fold
      (fun key value acc ->
        let show_sig = value |> signature_to_yojson |> Yojson.Safe.to_string in
        key ^ ":" ^ show_sig ^ "," ^ acc )
      all ""
end

let compute_signature (decl : Ast.decl) : string * signature =
  match decl with
  | Ast.Struct {name; fields} ->
      (name, StructSig {fields})
  | Ast.Enum {name} ->
      (name, EnumSig)
  | Ast.Class {name} ->
      (name, ClassSig)
  | Ast.Function {name; _} ->
      (name, FunctionSig)

let rec convert_program (program : Ast.program) : program =
  (* generate signatures for everything *)
  let env = TypeEnv.create () in
  List.iter
    (fun decl ->
      let name, sign = compute_signature decl in
      TypeEnv.add env name sign )
    program ;
  print_endline ("Env before typechecking:" ^ TypeEnv.pprint env) ;
  print_endline "---" ;
  List.map (convert_decl env) program

and convert_ty (env : TypeEnv.t) (ty : Ast.tylit) : ty =
  match ty with
  | Ast.TyName name ->
      ( match TypeEnv.lookup env name with
      | Some _ ->
          ()
      | None ->
          raise (Failure ("type " ^ name ^ " not found")) ) ;
      TyName name
  | TyInt ->
      TyInt

and convert_decl (env : TypeEnv.t) (decl : Ast.decl) : decl =
  match decl with
  | Ast.Struct {name; fields} ->
      Struct {name; fields}
  | Ast.Enum {name} ->
      Enum {name}
  | Ast.Class {name} ->
      Class {name}
  | Ast.Function {name; body} ->
      let body = convert_expr env body in
      let func = Function {name; body; ty= TypeEnv.generate ()} in
      func

and convert_stmt (env : TypeEnv.t) (stmt : Ast.stmt) : stmt =
  match stmt with
  | Ast.Let {lvalue; rvalue} ->
      let rvalue = convert_expr env rvalue in
      Let {lvalue; rvalue}
  | Ast.Expr expr ->
      let expr = convert_expr env expr in
      Expr expr

and convert_expr (env : TypeEnv.t) (expr : Ast.expr) : expr =
  match expr with
  | Ast.Unit ->
      Unit
  | Ast.Int n ->
      Int n
  | Ast.Seq (stmt, expr) ->
      let stmt = convert_stmt env stmt in
      let expr = convert_expr env expr in
      Seq (stmt, expr)
  | Ast.LValue lvalue ->
      LValue (lvalue, TypeEnv.generate ())
  | Ast.StructCon ty ->
      let ty = convert_ty env ty in
      StructCon ty
  | Ast.Call (func, arg) ->
      let func = convert_expr env func in
      let arg = convert_expr env arg in
      Call (func, arg, TypeEnv.generate ())

and get_expr_ty (env : TypeEnv.t) (expr : expr) : ty =
  match expr with
  | Unit ->
      TyUnit
  | Int _ ->
      TyInt
  | Seq (_, expr) ->
      get_expr_ty env expr
  | LValue (_, ty) ->
      ty
  | StructCon ty ->
      ty
  | Call (_, _, ty) ->
      ty
