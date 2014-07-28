extern crate xml;

use server_handler::XmppServerHandler;
use std::io::net::tcp::TcpStream;
use std::io::BufferedStream;

use read_str::ReadString;

use xmpp_socket::Tcp;
use ns;

/// Represent a server side XMPP stream
pub struct XmppServerStream {
    parser: xml::Parser,
    builder: xml::ElementBuilder,
    handler: XmppServerHandler,
}


impl XmppServerStream {

    pub fn new(
        stream: TcpStream
    ) -> XmppServerStream {
        XmppServerStream {
            parser: xml::Parser::new(),
            builder: xml::ElementBuilder::new(),
            handler: XmppServerHandler {
                socket: Tcp(BufferedStream::new(stream))
            }
        }
    }

    pub fn handle(&mut self) {

        loop {
            let string = {
                let socket = &mut self.handler.socket;
                match socket.read_str() {
                    Ok(string) => string,
                    Err(e) => {
                        println!("{}", e);
                        return;
                    }
                }
            };
            println!("raw string {}", string);
            self.parser.feed_str(string.as_slice());

            let builder = &mut self.builder;
            let handler = &mut self.handler;

            for event in self.parser { match event {
                Ok(xml::StartTag(xml::StartTag {
                    name: ref name,
                    ns: Some(ref ns),
                    prefix: ref prefix, ..
                })) if name.as_slice() == "stream" && ns.as_slice() == ns::STREAMS => {
                    println!("In: Stream start");
                    match *prefix {
                        Some(ref prefix) => {
                            *builder = xml::ElementBuilder::new();
                            builder.set_default_ns(ns::JABBER_CLIENT);
                            builder.define_prefix(prefix.as_slice(), ns::STREAMS);
                            handler.start_stream("localhost".as_slice());
                            handler.advertize_security_features();
                        },
                        None => ()
                    }
                },
                Ok(xml::EndTag(xml::EndTag {
                    name: ref name,
                    ns: Some(ref ns), ..
                })) if name.as_slice() == "stream" && ns.as_slice() == ns::STREAMS => {
                    println!("In: Stream end");
                    handler.close_stream();
                    return;
                }
               Ok(event) => {
                    match builder.push_event(event) {
                        Ok(Some(ref e)) => {
                            println!("In: {}", e)
                        }
                        Ok(None) => (),
                        Err(e) => println!("{}", e),
                    }
                },
                Err(e) => println!(
                    "Line: {} Column: {} Msg: {}",
                    e.line,
                    e.col, e.msg),
            };}
        }
    }

}
