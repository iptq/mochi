use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::str::FromStr;

use cranelift::prelude::{settings::Flags, *};
use cranelift_faerie::{FaerieBackend, FaerieBuilder, FaerieTrapCollection};
use cranelift_module::{Linkage, Module};

use crate::ast::{Expr, Func, Stmt};

pub struct Codegen {
    builder_ctx: FunctionBuilderContext,
    ctx: codegen::Context,
    module: Module<FaerieBackend>,
}

impl Codegen {
    pub fn new() -> Self {
        let mut flag_builder = settings::builder();
        flag_builder.enable("is_pic").unwrap();
        let isa_builder = isa::lookup(triple!("x86_64-unknown-unknown-elf")).unwrap();
        let isa = isa_builder.finish(Flags::new(flag_builder));

        let builder = FaerieBuilder::new(
            isa,
            "mochi".to_string(),
            FaerieTrapCollection::Disabled,
            FaerieBuilder::default_libcall_names(),
        )
        .expect("failed");
        let module = Module::new(builder);
        let builder_ctx = FunctionBuilderContext::new();
        let ctx = module.make_context();
        Self {
            builder_ctx,
            ctx,
            module,
        }
    }

    pub fn compile_func(&mut self, func: &Func) {
        let int = self.module.target_config().pointer_type();
        self.ctx.func.signature.returns.push(AbiParam::new(int));
        let mut builder = FunctionBuilder::new(&mut self.ctx.func, &mut self.builder_ctx);
        let entry_ebb = builder.create_ebb();
        builder.append_ebb_params_for_function_params(entry_ebb);
        builder.switch_to_block(entry_ebb);
        builder.seal_block(entry_ebb);
        let mut trans = FunctionTranslator {
            builder,
            module: &mut self.module,
        };
        for stmt in &func.body {
            trans.translate_stmt(stmt);
        }
        trans.builder.finalize();

        let id = self
            .module
            .declare_function(&func.name, Linkage::Export, &self.ctx.func.signature)
            .map_err(|e| e.to_string())
            .expect("failed");
        self.module
            .define_function(id, &mut self.ctx)
            .map_err(|e| e.to_string())
            .expect("failed");
        self.module.clear_context(&mut self.ctx);
        self.module.finalize_definitions();
        let code = self.module.get_finalized_function(id);
        println!("code: {:?}", code);
    }

    pub fn finish(self, path: impl AsRef<Path>) {
        let file = File::create(path).expect("failed");
        let product = self.module.finish();
        product.write(file);
    }
}

pub struct FunctionTranslator<'a> {
    builder: FunctionBuilder<'a>,
    module: &'a mut Module<FaerieBackend>,
}

impl<'a> FunctionTranslator<'a> {
    pub fn translate_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Expr(expr) => {
                self.translate_expr(expr);
            }
            Stmt::Return(expr) => {
                let v = self.translate_expr(expr);
                self.builder.ins().return_(&[v]);
            }
            Stmt::If(cond, tbody, fbody) => {
                self.translate_expr(cond);
                // TODO: finish
            }
        }
    }

    pub fn translate_expr(&mut self, expr: &Expr) -> Value {
        match expr {
            Expr::Int(n) => {
                let int = self.module.target_config().pointer_type();
                self.builder.ins().iconst(int, *n)
            }
            Expr::Call(func, args) => {
                let int = self.module.target_config().pointer_type();
                self.builder.ins().iconst(int, 117)
            }
        }
    }
}
