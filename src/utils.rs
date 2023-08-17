// panic hook for error functions: if used, program will handle panics macros when triggered.
// to activate, include this in our lib.rs 
//https://rustwasm.github.io/wasm-pack/book/tutorials/npm-browser-packages/testing-your-project.html

/*
 pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
 */