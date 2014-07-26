extern crate xmpp;
use xmpp::XmppServerListener;
use xmpp::XmppServerStream;

fn main() {

    let mut serverStream = XmppServerListener::new("127.0.0.1", 5222);
    serverStream.listen();
}

