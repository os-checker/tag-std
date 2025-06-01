#![feature(stmt_expr_attributes)]
#![feature(register_tool)]
#![register_tool(Safety)]
#![allow(dead_code)]

pub fn tag_expr() {
    unsafe {
        #[Safety::tag_expr]
        call()
    };
}

pub fn tag_block() {
    #[Safety::tag_block]
    unsafe {
        call();
    }
}

#[Safety::inner(Tag)]
unsafe fn call() {}

#[Safety::tag_unsafe_fn]
unsafe fn tag_unsafe_fn() {
    call();
}

pub fn assign() {
    let f = call;
    #[Safety::assign]
    unsafe {
        f()
    };
}

pub fn assign_fn_ptr() {
    let f: unsafe fn() = call;
    unsafe {
        #[Safety::assign_fn_ptr]
        f()
    };
}

pub fn no_tag() {
    unsafe { call() };
}

mod submod {
    unsafe fn submod_no_tag() {
        super::call();
    }
}
