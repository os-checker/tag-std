use rustc_hir::{intravisit::*, *};
use rustc_middle::ty::TyCtxt;

pub struct UnsafeBlocks<'tcx> {
    tcx: TyCtxt<'tcx>,
    blocks: Vec<HirId>,
}

impl<'tcx> Visitor<'tcx> for UnsafeBlocks<'tcx> {
    type MaybeTyCtxt = TyCtxt<'tcx>;
    type NestedFilter = rustc_middle::hir::nested_filter::OnlyBodies;
    type Result = ();

    fn maybe_tcx(&mut self) -> Self::MaybeTyCtxt {
        self.tcx
    }

    fn visit_block(&mut self, b: &'tcx Block<'tcx>) -> Self::Result {
        if matches!(b.rules, BlockCheckMode::UnsafeBlock(_)) {
            self.blocks.push(b.hir_id);
        }
        walk_block(self, b)
    }
}

pub fn get_unsafe_blocks<'tcx>(tcx: TyCtxt<'tcx>, expr: &'tcx Expr<'tcx>) -> Vec<HirId> {
    let mut visitor = UnsafeBlocks { tcx, blocks: Vec::new() };
    walk_expr(&mut visitor, expr);
    visitor.blocks
}
