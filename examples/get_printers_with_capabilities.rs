use printers::get_printers_with_capabilities;

fn main() {
    let printers = get_printers_with_capabilities();
    println!("{:?}", printers);
}
