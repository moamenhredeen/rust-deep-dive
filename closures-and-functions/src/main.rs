use std::ops::Add;

#[allow(unused)]
fn add<T>(a: T, b: T) -> T
where
    T: Add<Output = T>,
{
    a + b
}

fn hook(callback: fn(u32, u32) -> u32, a: u32, b: u32) -> u32 {
    callback(a, b)
}

fn hoc_fn<F: Fn()>(closure: &F) {
    closure()
}

fn hoc_fn_mut<F: FnMut()>(closure: &mut F) {
    closure()
}

fn hoc_fn_once<F: FnOnce()>(closure: F) {
    closure()
}

fn main() {
    let add_function_item = add::<u32>;
    let res = add_function_item(2, 2);
    println!("add_function_item = {}", res);

    let add_callback = add;
    let res = hook(add_callback, 2, 2);
    println!("hook = {}", res);

    print!("\n\n-----------------------------------------------\n\n");

    let mut closure_impl_fn_and_fn_mut_and_fn_once =
        || println!("this is a closure that implement Fn, FnMut, FnOnce");
    hoc_fn(&closure_impl_fn_and_fn_mut_and_fn_once);
    hoc_fn_mut(&mut closure_impl_fn_and_fn_mut_and_fn_once);
    hoc_fn_once(closure_impl_fn_and_fn_mut_and_fn_once);

    print!("\n\n-----------------------------------------------\n\n");

    let mut captured_by_the_closure = String::from("hello world");
    let mut closure_impl_fn_mut_and_fn_once = || {
        captured_by_the_closure.clear();
        println!("this is a closure that implement FnMut, FnOnce");
    };
    // hoc_mut_fn(&mut xx)
    hoc_fn_mut(&mut closure_impl_fn_mut_and_fn_once);
    hoc_fn_once(closure_impl_fn_mut_and_fn_once);

    print!("\n\n-----------------------------------------------\n\n");

    let moved_to_closure = String::from("hello world");
    let closure_impl_fn_once = || {
        drop(moved_to_closure);
        println!("this is a closure that implement FnOnce");
    };
    // hoc_mut_fn(&mut xx)
    hoc_fn_once(closure_impl_fn_once);

}
