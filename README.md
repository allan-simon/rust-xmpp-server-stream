Rust XMPP server stream

library made to handle the part of XMPP defined in RFC3290
(i.e core of the protocol)

The goal being to permit the library to accept "plugins" or to define only Traits for
everything else.

List of things this library will handle:

  * creating a listening for incoming connections
  * poping a `proc` for every single incoming connection
  * handling the XMPP stream start 
  * handling the optional switch to TLS
  * handling the SASL authentication part
  * handling the resource binding part

Things that will not be handle and only provided as "trait"

  * non widely spread SASL method (we will certainly only provide 'PLAIN')
    and let an interface to register additional methods
  * account storer, we will only provide the interfaces to check a triple
    {domain,username,password} but let the actual implementation up to the user
    (SQL database, json file etc.)
  * session handling
  * post resource binding stanza handling (for this we will certainly adopt
    an interface 'a la Prosody' where third party plugins register to some
    kind of 'stanza signature' (e.g the plugin A will listen to all IQ
    with a child query and all message of type 'chat')

##How to compile

  cargo build

an example of code using it is provided in `example` folder

##Note

For the moment a huge part of the code is a shameless copy/paste of
[Florob's rust xmpp client library](https://github.com/Florob/rust-xmpp)


##Licence

The code is under MIT licence
