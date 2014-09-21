extern crate xmpp;
use xmpp::XmppStream;

fn main() {
    
    let mut stream = XmppStream::new("1608427-92", "chat.quickblox.com", "rustuser");
    
    println!("Connetiing...");

    match stream.connect() {
        Ok(_) => {
            println!("Connected");
        },
        Err(e) => {
            println!("Error while connecting: {}", e);
            return;
        }
    }
    println!("{}", stream.handle());
}
