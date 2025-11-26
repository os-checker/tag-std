use super::{Tag, TagType};

/// `any` tag is denied in user's spec, and special in doc generation.
pub const ANY: &str = "any";
/// `CallOnce` tag is builtin. Users can't define it.
pub const POST_TO_FUNC: &str = "PostToFunc";

/// Returns true if the tag is builtin.
pub fn is_builtin_tag(name: &str) -> bool {
    matches!(name, ANY | POST_TO_FUNC)
}

/// Instances of builtin tags. `any` is excluded.
pub fn tags() -> Vec<(&'static str, Tag)> {
    vec![(POST_TO_FUNC, post_to_func())]
}

fn post_to_func() -> Tag {
    Tag {
        args: Box::new(["func".into()]),
        desc: Some("This function must be called after {func}.".into()),
        expr: None,
        types: Box::new([TagType::Precond]),
        url: Some("https://github.com/Artisan-Lab/tag-std/blob/main/Asterinas-safety-properties.md#11-posttofuncfunc".into()),
    }
}
