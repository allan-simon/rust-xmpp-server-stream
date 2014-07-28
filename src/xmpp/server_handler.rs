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

    /// start XMPP stream from server side
    ///
    /// to be called after we've received <stream:stream> from client
    /// as defined in RFC3920
    /// http://xmpp.org/rfcs/rfc3920.html#streams
    ///
    /// domain is the domain served by this connection
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

    /// advertize the security features provided by the server
    ///
    /// must be send right after the server has sent its own
    /// <stream:stream> (without waiting for client answer)
    /// and BEFORE non-security features are advertized, as define
    /// in RFC3920 section 4.6
    pub fn advertize_security_features(&mut self) -> IoResult<()> {

        // TODO the list of SASL mechanism provided should be extensible
        // rather than hardcoded
        let features = format!(
            "<stream:features>\
                <mechanisms xmlns='{sasl}'>\
                    <mechanism>PLAIN</mechanism>\
                </mechanisms>\
            </stream:features>",
            sasl= ns::FEATURE_SASL
        );

        self.send(features)
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


