use rustc_hir::{
    def::{DefKind, Res},
    def_id::DefId,
    intravisit::*,
    *,
};
use rustc_middle::ty::TyCtxt;

#[derive(Debug)]
pub struct Call {
    /// function use id
    pub hir_id: HirId,
    /// function def id
    pub def_id: DefId,
}

pub struct Calls<'tcx> {
    tcx: TyCtxt<'tcx>,
    calls: Vec<Call>,
}

impl<'tcx> Visitor<'tcx> for Calls<'tcx> {
    type MaybeTyCtxt = TyCtxt<'tcx>;
    type NestedFilter = rustc_middle::hir::nested_filter::OnlyBodies;
    type Result = ();

    fn maybe_tcx(&mut self) -> Self::MaybeTyCtxt {
        self.tcx
    }

    fn visit_expr(&mut self, ex: &'tcx Expr<'tcx>) -> Self::Result {
        if let ExprKind::Path(QPath::Resolved(_opt_ty, path)) = ex.kind {
            if let Res::Def(DefKind::Fn, def_id) = path.res {
                self.calls.push(Call { hir_id: ex.hir_id, def_id });
            }
        }
        walk_expr(self, ex)
    }
}

pub fn get_calls<'tcx>(tcx: TyCtxt<'tcx>, expr: &'tcx Expr<'tcx>) -> Calls<'tcx> {
    let mut calls = Calls { tcx, calls: Vec::new() };
    walk_expr(&mut calls, expr);
    calls
}

impl Calls<'_> {
    pub fn get_unsafe_calls(&self) -> Vec<&Call> {
        self.calls
            .iter()
            .filter(|call| self.tcx.fn_sig(call.def_id).skip_binder().safety().is_unsafe())
            .collect()
    }
}
