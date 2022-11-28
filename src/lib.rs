use swc_core::common::util::take::Take;
use swc_core::common::Span;
use swc_core::ecma::ast::{BlockStmt, ExprOrSpread, ExprStmt, ReturnStmt, Stmt};
use swc_core::ecma::{
    ast::{Expr, Program},
    transforms::testing::test,
    visit::{as_folder, FoldWith, VisitMut},
};
use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};

pub struct DefineRender;

impl DefineRender {
    fn is_define_render(&self, expr_stmt: &ExprStmt) -> bool {
        if let Expr::Call(call_expr) = &*expr_stmt.expr {
            if let Some(expr) = call_expr.callee.as_expr() {
                if let Expr::Ident(ident) = &**expr {
                    if ident.sym.eq("defineRender") {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn get_define_render_body(&self, expr_stmt: &ExprStmt) -> Option<ExprOrSpread> {
        if let Expr::Call(call_expr) = &*expr_stmt.expr {
            if let Some(expr) = call_expr.callee.as_expr() {
                if let Expr::Ident(ident) = &**expr {
                    if ident.sym.eq("defineRender") {
                        return Some(call_expr.args[0].clone());
                    }
                }
            }
        }
        None
    }

    fn remove_return_if_exist(&self, stmt: &mut Stmt) {
        if let Stmt::Return(_) = stmt {
            stmt.take();
        }
    }
}

impl VisitMut for DefineRender {
    fn visit_mut_block_stmt(&mut self, block: &mut BlockStmt) {
        let stmts = &mut block.stmts;
        let mut flag = false;
        let mut render_statement: Option<ExprOrSpread> = None;
        for stmt in stmts.into_iter() {
            if let Some(expr_stmt) = stmt.as_expr() {
                if self.is_define_render(expr_stmt) {
                    render_statement = self.get_define_render_body(expr_stmt);
                    stmt.take();
                    flag = true;
                }
            }

            if flag {
                self.remove_return_if_exist(stmt);
            }
        }
        if flag {
            if let Some(render_statement) = render_statement {
                stmts.push(Stmt::Return(ReturnStmt {
                    span: Span::default(),
                    arg: Some(render_statement.expr),
                }));
            }
        }
    }
}

/// Refer swc_plugin_macro to see how does it work internally.
#[plugin_transform]
pub fn process_transform(program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
    program.fold_with(&mut as_folder(DefineRender))
}

test!(
    Default::default(),
    |_| as_folder(DefineRender),
    boo,
    // Input codes
    r#"
    import { defineComponent as _defineComponent } from 'vue'
    import { openBlock as _openBlock, createElementBlock as _createElementBlock } from "vue"
    import { h } from 'vue'
    
    export default _defineComponent({
      __name: 'basic',
      setup(__props) {
        defineRender(() => h('div'))
        return (_ctx, _cache) => {
            return _openBlock(), _createElementBlock("div")
        }
      }
    })
    "#,
    // Output codes after transformed with plugin
    r#"
    import { defineComponent as _defineComponent } from 'vue'
    import { openBlock as _openBlock, createElementBlock as _createElementBlock } from "vue"
    import { h } from 'vue'
    
    export default _defineComponent({
      __name: 'basic',
      setup(__props) {
        ;;
        return () => h('div')
      }
    })
    "#
);
