#[nuclei_attributes::main]
async fn main() {
    nuclei::spawn(async {
        println!("Hello, world!");
    })
    .await;
}