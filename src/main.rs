use scaleway::{Client, TokenId};

fn main() {
    let client = Client::from_token(env!("SCW_TOKEN")).unwrap();
    let (id, token) = client.create_token("xx", "xx", None).unwrap();
    println!("{:#?}", token);
    println!("{:#?}", client.token(id));

}
