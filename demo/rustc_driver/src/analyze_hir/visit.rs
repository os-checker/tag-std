use super::{FnToolAttrs, SafetyAttr, is_tool_attr};
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
    pub fn get_all_attrs(&self, fn_hir_id: HirId, safety_attrs: &mut FnToolAttrs) {
        let tcx = safety_attrs.tcx;
        let tags = safety_attrs.get_or_insert_tags(self.def_id);

        let mut print = |hir_id: HirId| {
            eprintln!("hir_id={hir_id:?} fn_hir_id={fn_hir_id:?}");

            let attrs: Vec<_> = tcx.hir_attrs(hir_id).iter().filter(is_tool_attr).collect();
            for attr in &attrs {
                eprintln!("{hir_id:?} {}", attribute_to_string(&tcx, attr));
                let tag = SafetyAttr::new(attr)
                    .unwrap_or_else(|| panic!("{attr:?} should contain an Ident to discharge"))
                    .property;
                let Some(state) = tags.get_mut(&tag) else {
                    panic!("tag {tag} doesn't belong to tags {tags:?}")
                };
                assert!(!*state, "{tag} has already been discharged");
                *state = true;
            }
            for (tag, state) in &*tags {
                assert!(*state, "{tag:?} is not discharged");
            }
            attrs.is_empty()
        };
        print(self.hir_id);

        for parent in tcx.hir_parent_id_iter(self.hir_id) {
            let empty = print(parent);
            // Stop at first tool attrs or the function item.
            // For a function inside a nested module, hir_parent_id_iter
            // will pop up to the crate root, thus it's necessary to
            // stop when reaching the fn item.
            if !empty || parent == fn_hir_id {
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
