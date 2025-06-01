#![feature(stmt_expr_attributes)]
#![feature(register_tool)]
#![register_tool(Safety)]
#![allow(dead_code)]

pub fn tag_expr() {
    unsafe {
        #[Safety::tag_expr(Tag)]
        call()
    };
}

pub fn tag_block() {
    #[Safety::tag_block(Tag)]
    unsafe {
        call();
    }
}

#[Safety::inner(Tag)]
unsafe fn call() {}

#[Safety::tag_unsafe_fn(Tag)]
unsafe fn tag_unsafe_fn() {
    call();
}
