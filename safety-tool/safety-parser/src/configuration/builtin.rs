use super::{Tag, TagType};

/// `any` tag is denied in user's spec, and special in doc generation.
pub const ANY: &str = "any";
/// `CallOnce` tag is builtin. Users can't define it.
pub const CALL_ONCE: &str = "CallOnce";

/// Returns true if the tag is builtin.
pub fn is_builtin_tag(name: &str) -> bool {
    matches!(name, ANY | CALL_ONCE)
}

/// Instances of builtin tags. `any` is excluded.
pub fn tags() -> Vec<(&'static str, Tag)> {
    vec![(CALL_ONCE, call_once())]
}

fn call_once() -> Tag {
    Tag {
        args: Box::new(["func".into()]),
        desc: Some("The `{func}` must be called only once.".into()),
        expr: None,
        types: Box::new([ TagType::Precond]),
        url: Some("https://github.com/Artisan-Lab/tag-std/blob/main/Asterinas-safety-properties.md#14-calloncescope".into()),
    }
}
