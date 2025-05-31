use rustc_hir::{def_id::DefId, intravisit::Visitor};
use rustc_middle::ty::TyCtxt;

struct Call<'tcx> {
    tcx: TyCtxt<'tcx>,
    /// caller
    caller: DefId,
    /// is the caller an unsafe fn?
    unsafe_: bool,
    /// direct callees in the body
    callees: Vec<Call<'tcx>>,
}

impl<'tcx> Visitor<'tcx> for Call<'tcx> {
    type MaybeTyCtxt = TyCtxt<'tcx>;
    type NestedFilter = rustc_middle::hir::nested_filter::OnlyBodies;
    type Result = ();

    fn visit_fn(
        &mut self,
        fk: rustc_hir::intravisit::FnKind<'tcx>,
        fd: &'tcx rustc_hir::FnDecl<'tcx>,
        b: rustc_hir::BodyId,
        _: rustc_span::Span,
        id: rustc_hir::def_id::LocalDefId,
    ) -> Self::Result {
    }

    fn visit_body(&mut self, b: &rustc_hir::Body<'tcx>) -> Self::Result {}
}
