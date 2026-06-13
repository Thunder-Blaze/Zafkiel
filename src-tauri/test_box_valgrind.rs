struct GlContext {
    display: u32,
    context: String,
}

fn main() {
    let b = Box::new(GlContext { display: 123, context: "hello".to_string() });
    let c = b.context;
    
    // Attempt to write to display via pointer.
    // If the heap was freed, valgrind will complain about invalid write.
    let addr = &b.display as *const u32 as *mut u32;
    unsafe { *addr = 456; }
    println!("Value: {}", unsafe { *addr });
}
