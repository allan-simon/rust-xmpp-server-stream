use std::io::IoResult;

use xmpp_send::XmppSend;
use xmpp_socket::XmppSocket;
use ns;


///
///
///
pub struct XmppServerHandler {
    pub socket: XmppSocket
}


///
///
///
impl XmppServerHandler {
    pub fn start_stream(&mut self, domain: &str) -> IoResult<()> {
        let start = format!(
            "<?xml version='1.0'?>\n\
            <stream:stream \
                xmlns:stream='{}'
                xmlns='{}' \
                version='1.0' \
                from='{}'\
                id='{}'
            >",
            ns::STREAMS,
            ns::JABBER_CLIENT,
            domain,
            "arandomid"
        );
        self.send(start)
    }

    pub fn close_stream(&mut self) -> IoResult<()> {
        self.send("</stream:stream>")
    }

    pub fn send<T: XmppSend>(&mut self, data: T) -> IoResult<()> {
        let data = data.xmpp_str();
        println!("Out: {}", data);
        try!(self.socket.write(data.as_slice().as_bytes()));
        self.socket.flush()
    }
}


