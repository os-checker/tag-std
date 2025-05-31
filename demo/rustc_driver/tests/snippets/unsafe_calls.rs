#![allow(dead_code)]

pub fn foo() {
    unsafe { call() };
}

pub fn bar() {
    unsafe {
        call();
    }
}

unsafe fn call() {}

unsafe fn baz() {
    call();
}

pub fn assign() {
    let f = call;
    unsafe { f() };
}

pub fn assign_fn_ptr() {
    let f: unsafe fn() = call;
    unsafe { f() };
}
