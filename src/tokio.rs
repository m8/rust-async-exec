
async fn say_haha() {
    println!("haha");
}

#[tokio::main]
pub async fn tokio_examples() {
    
    tokio::spawn(say_haha());

}