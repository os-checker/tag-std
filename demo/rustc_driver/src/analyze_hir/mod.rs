use crate::REGISTER_TOOL;
use rustc_ast::MetaItemInner;
use rustc_data_structures::fx::FxIndexMap;
use rustc_hir::{
    AttrItem, Attribute, BodyId, FnSig, HirId, ImplItemKind, ItemKind, Node, def_id::DefId,
};
use rustc_middle::ty::TyCtxt;
use rustc_span::{Ident, Span};

mod db;
mod visit;

pub fn analyze_hir(tcx: TyCtxt) {
    let mut safety_attrs = FnToolAttrs::new(tcx);

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

        let tool_attrs = attrs.iter().filter(is_tool_attr);

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

        eprintln!("{}", rustc_hir_pretty::id_to_string(&tcx, hir_fn.hir_id));

        // look in the body
        let body = tcx.hir_body(hir_fn.body).value;
        let calls = visit::get_calls(tcx, body);
        let unsafe_calls = calls.get_unsafe_calls();
        if !unsafe_calls.is_empty() {
            dbg!(&unsafe_calls);
            for call in &unsafe_calls {
                call.get_all_attrs(hir_fn.hir_id, &mut safety_attrs);
            }
        }
    }
}

fn is_tool_attr(attr: &&Attribute) -> bool {
    if let Attribute::Unparsed(tool_attr) = attr {
        if tool_attr.path.segments[0].as_str() == REGISTER_TOOL {
            return true;
        }
    }
    false
}

struct HirFn<'hir> {
    hir_id: HirId,
    name: Ident,
    sig: FnSig<'hir>,
    body: BodyId,
    span: Span,
}

struct FnToolAttrs<'tcx> {
    tcx: TyCtxt<'tcx>,
    map: FxIndexMap<DefId, Vec<SafetyAttr<'tcx>>>,
    /// State of safety tags shows if thet are discharged.
    tagged: FxIndexMap<Ident, bool>,
}

impl<'tcx> FnToolAttrs<'tcx> {
    fn new(tcx: TyCtxt<'tcx>) -> Self {
        Self { tcx, map: FxIndexMap::default(), tagged: FxIndexMap::default() }
    }

    fn get_or_insert(&mut self, did: DefId) -> &[SafetyAttr<'tcx>] {
        self.map
            .entry(did)
            .or_insert_with(|| self.tcx.get_all_attrs(did).filter_map(SafetyAttr::new).collect())
    }

    fn get_or_insert_tags(&mut self, did: DefId) -> &mut FxIndexMap<Ident, bool> {
        self.tagged.clear();
        let tags: Vec<_> =
            self.get_or_insert(did).iter().map(|attr| (attr.property, false)).collect();
        self.tagged.extend(tags);
        &mut self.tagged
    }
}

struct SafetyAttr<'tcx> {
    attr_item: &'tcx AttrItem,
    property: Ident,
}

impl<'tcx> SafetyAttr<'tcx> {
    fn new(attr: &Attribute) -> Option<SafetyAttr> {
        if let MetaItemInner::MetaItem(meta) = attr.meta_item_list()?.first()? {
            // #[Safety::path(Property)]
            let property = dbg!(meta).ident()?;
            dbg!(property);
            if let Attribute::Unparsed(tool_attr) = attr {
                if tool_attr.path.segments[0].as_str() == REGISTER_TOOL {
                    return Some(SafetyAttr { attr_item: tool_attr, property });
                }
            }
        }
        None
    }
}
