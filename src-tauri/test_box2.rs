struct GlContext {
    display: u32,
    context: String,
}

fn main() {
    let b = Box::new(GlContext { display: 123, context: "hello".to_string() });
    let c = b.context; // Partial move out of box
}
