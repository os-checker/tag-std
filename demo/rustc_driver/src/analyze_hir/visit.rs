use rustc_hir::{intravisit::*, *};
use rustc_middle::ty::TyCtxt;

#[derive(Debug)]
pub struct BlockAndUnsafeCalls {
    pub id: HirId,
    pub calls: Vec<HirId>,
}

struct UnsafeBlocks<'tcx> {
    tcx: TyCtxt<'tcx>,
    blocks: Vec<BlockAndUnsafeCalls>,
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
            let mut unsafe_calls = UnsafeCalls { tcx: self.tcx, calls: Vec::new() };
            walk_block(&mut unsafe_calls, b);

            let b_calls = BlockAndUnsafeCalls { id: b.hir_id, calls: unsafe_calls.calls };
            self.blocks.push(b_calls);
        }
        walk_block(self, b)
    }
}

pub fn get_unsafe_blocks<'tcx>(
    tcx: TyCtxt<'tcx>,
    expr: &'tcx Expr<'tcx>,
) -> Vec<BlockAndUnsafeCalls> {
    let mut visitor = UnsafeBlocks { tcx, blocks: Vec::new() };
    walk_expr(&mut visitor, expr);
    visitor.blocks
}

struct UnsafeCalls<'tcx> {
    tcx: TyCtxt<'tcx>,
    calls: Vec<HirId>,
}

impl<'tcx> Visitor<'tcx> for UnsafeCalls<'tcx> {
    type MaybeTyCtxt = TyCtxt<'tcx>;
    type NestedFilter = rustc_middle::hir::nested_filter::OnlyBodies;
    type Result = ();

    fn maybe_tcx(&mut self) -> Self::MaybeTyCtxt {
        self.tcx
    }

    fn visit_expr(&mut self, ex: &'tcx Expr<'tcx>) -> Self::Result {
        if let ExprKind::Call(f, _) = ex.kind {
            self.calls.push(f.hir_id);
        }
        walk_expr(self, ex)
    }
}
