use swc_common::{util::take::Take, DUMMY_SP};
use swc_ecma_ast::*;
use swc_ecma_utils::contains_this_expr;

use super::Pure;
use crate::mode::Mode;

/// Methods related to the option `arrows`.
impl<M> Pure<'_, M>
where
    M: Mode,
{
    pub(super) fn optimize_arrow_body(&mut self, b: &mut BlockStmtOrExpr) {
        if !self.options.arrows {
            return;
        }

        match b {
            BlockStmtOrExpr::BlockStmt(s) => {
                if s.stmts.len() == 1 {
                    if let Stmt::Return(s) = &mut s.stmts[0] {
                        if let Some(arg) = &mut s.arg {
                            tracing::debug!("arrows: Optimizing the body of an arrow");
                            *b = BlockStmtOrExpr::Expr(arg.take());
                        }
                    }
                }
            }
            BlockStmtOrExpr::Expr(_) => {}
        }
    }

    pub(super) fn optimize_arrow_method_prop(&mut self, p: &mut Prop) {
        if self.options.ecma < EsVersion::Es2015 {
            return;
        }

        if !self.options.unsafe_methods && !self.options.arrows {
            return;
        }

        if let Prop::KeyValue(kv) = p {
            //
            if contains_this_expr(&kv.value) {
                return;
            }

            if let Expr::Arrow(
                m @ ArrowExpr {
                    body: BlockStmtOrExpr::BlockStmt(..),
                    ..
                },
            ) = &mut *kv.value
            {
                *p = Prop::Method(MethodProp {
                    key: kv.key.take(),
                    function: Function {
                        params: m
                            .params
                            .take()
                            .into_iter()
                            .map(|pat| Param {
                                span: DUMMY_SP,
                                decorators: Default::default(),
                                pat,
                            })
                            .collect(),
                        decorators: Default::default(),
                        span: m.span,
                        body: m.body.take().block_stmt(),
                        is_generator: m.is_generator,
                        is_async: m.is_async,
                        type_params: Default::default(),
                        return_type: Default::default(),
                    },
                });
            }
        }
    }
}
