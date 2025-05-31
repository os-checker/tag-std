use crate::REGISTER_TOOL;
use rustc_hir::{Attribute, BodyId, Expr, ExprKind, FnSig, HirId, ImplItemKind, ItemKind, Node};
use rustc_middle::ty::{Instance, TyCtxt};
use rustc_span::{Ident, Span};

mod db;
mod reachability;
mod visit;

pub fn analyze_hir(tcx: TyCtxt) {
    let def_items = tcx.hir_crate_items(()).definitions();
    for local_def_id in def_items {
        let node = tcx.hir_node_by_def_id(local_def_id);

        // fn item or associated fn item
        let hir_fn = match node {
            Node::Item(item) if matches!(item.kind, ItemKind::Fn { .. }) => {
                let (name, sig, _generics, body) = item.expect_fn();
                let sig = *sig;
                HirFn { hir_id: item.hir_id(), name, sig, body, span: item.vis_span }
            }
            Node::ImplItem(item) if matches!(item.kind, ImplItemKind::Fn(..)) => {
                let (sig, body) = item.expect_fn();
                HirFn { hir_id: item.hir_id(), name: item.ident, sig: *sig, body, span: item.span }
            }
            _ => continue,
        };

        let attrs = tcx.hir_attrs(hir_fn.hir_id);

        let tool_attrs = attrs.iter().filter(|attr| {
            if let Attribute::Unparsed(tool_attr) = attr {
                if tool_attr.path.segments[0].as_str() == REGISTER_TOOL {
                    return true;
                }
            }
            false
        });

        let def_id = local_def_id.to_def_id();
        let def_path = tcx.def_path_debug_str(def_id);
        let hash = tcx.def_path_hash(def_id).0;

        for attr in tool_attrs {
            println!(
                "+++++ {fn_name} (def_path={def_path:?} hash={hash}) ({span:?}) +++++\n => {attr:?}\n",
                fn_name = hir_fn.name,
                span = attr.span(),
                attr = rustc_hir_pretty::attribute_to_string(&tcx, attr)
            );
        }

        // look in the body
        let body = tcx.hir_body(hir_fn.body).value;
        let unsafe_blocks = visit::get_unsafe_blocks(tcx, body);
        if !unsafe_blocks.is_empty() {
            dbg!(&unsafe_blocks);
            for b in &unsafe_blocks {
                for call in &b.calls {
                    dbg!(tcx.hir_expect_expr(*call));
                }
            }
        }
    }
}

struct HirFn<'hir> {
    hir_id: HirId,
    name: Ident,
    sig: FnSig<'hir>,
    body: BodyId,
    span: Span,
}
