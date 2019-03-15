use scaleway::Client;

fn main() {
    let client = Client::from_token(env!("SCW_TOKEN")).unwrap();
    println!("{:#?}", client.tokens());
}
