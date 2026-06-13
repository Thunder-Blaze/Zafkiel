struct GlContext {
    display: u32,
    context: String,
}

impl Drop for GlContext {
    fn drop(&mut self) {
        println!("GlContext dropped!");
    }
}

fn main() {
    let b = Box::new(GlContext { display: 123, context: "hello".to_string() });
    let c = b.context;
    let addr = &b.display as *const _;
    println!("{:?}", addr);
}
