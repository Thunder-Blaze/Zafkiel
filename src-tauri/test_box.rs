struct GlContext {
    display: u32,
    context: String,
}

fn main() {
    let b = Box::new(GlContext { display: 123, context: "hello".to_string() });
    
    // Partially move
    let c = b.context;
    
    // Address of display
    let addr = &b.display as *const _;
    println!("{:?}", addr);
}
