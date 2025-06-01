use super::is_tool_attr;
use rustc_hir::{
    def::{DefKind, Res},
    def_id::DefId,
    intravisit::*,
    *,
};
use rustc_hir_pretty::attribute_to_string;
use rustc_middle::ty::TyCtxt;

#[derive(Debug)]
pub struct Call {
    /// function use id
    pub hir_id: HirId,
    /// function def id
    pub def_id: DefId,
}

impl Call {
    pub fn get_all_attrs(&self, body_hir_id: HirId, tcx: TyCtxt) {
        let print = |hir_id: HirId| {
            eprintln!("hir_id={hir_id:?} body_hir_id={body_hir_id:?}");
            let mut empty = true;
            for attr in tcx.hir_attrs(hir_id).iter().filter(is_tool_attr) {
                eprintln!("{hir_id:?} {}", attribute_to_string(&tcx, attr));
                empty = false;
            }
            empty
        };
        print(self.hir_id);

        for parent in tcx.hir_parent_id_iter(self.hir_id) {
            let empty = print(parent);
            // stop at first tool attrs
            if !empty || parent == body_hir_id {
                break;
            }
        }
    }
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
