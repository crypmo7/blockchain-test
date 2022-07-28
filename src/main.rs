mod chain;
use chain::chain::App;
fn main() {
    let mut new_app = App::new();
    new_app.genesis();
    new_app.add_new_block("block1".to_string());
    new_app.add_new_block("block2".to_string());
    new_app.add_new_block("block3".to_string());
    println!("{:#?}", new_app);
}
