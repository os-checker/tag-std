#![feature(stmt_expr_attributes)]
#![feature(register_tool)]
#![register_tool(Safety)]
#![allow(dead_code)]

pub fn tag_expr() {
    unsafe {
        #[Safety::tag_expr(property = Memo(Tag), kind = "memo")]
        call()
    };
}

pub fn tag_block() {
    #[Safety::tag_block(property = Memo(Tag), kind = "memo")]
    unsafe {
        call();
    }
}

#[Safety::inner(property = Memo(Tag), kind = "memo")]
unsafe fn call() {}

#[Safety::tag_unsafe_fn(property = Memo(Tag), kind = "memo")]
pub unsafe fn tag_unsafe_fn() {
    call();
}
