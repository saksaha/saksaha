use std::sync::Arc;

use tokio::sync::Mutex;

use crate::p2p::peer::Peer;

pub enum HandleStatus {
    Success,
}

pub struct Handler {
    peer: Arc<Mutex<Peer>>,
    // credential: Arc<Credential>,
    // peer_op_port: u16,
    // address_book: Arc<AddressBook>,
    // my_disc_endpoint: String,
    // peer_op_wakeup_tx: Arc<Sender<usize>>,
}

impl Handler {
    pub fn new(
        peer: Arc<Mutex<Peer>>,
        // credential: Arc<Credential>,
        // peer_op_port: u16,
        // address_book: Arc<AddressBook>,
    ) -> Handler {
        Handler {
            peer,
        }
    }

    pub async fn run(&self) -> HandleStatus {
        let peer = self.peer.lock().await;
        // peer.
        // let address_book_len = self.address_book.len().await;

        // log!(DEBUG, "Address book len: {}\n", address_book_len);

        // let (addr, idx) =
        //     match self.address_book.next(&Filter::not_discovered).await {
        //         Some(a) => a,
        //         None => {
        //             log!(DEBUG, "Cannot acquire next address\n");

        //             return HandleStatus::NoAvailableAddress;
        //         }
        //     };

        // let addr = addr.lock().await;

        // if addr.endpoint == self.my_disc_endpoint {
        //     match self.handle_my_endpoint(addr.endpoint.to_owned(), idx).await {
        //         Ok(_) => return HandleStatus::LocalAddrIdentical,
        //         Err(err) => {
        //             log!(DEBUG, "Error handling my endpoint, err: {}", err);
        //         }
        //     }
        // };

        // let mut stream =
        //     match TcpStream::connect(addr.endpoint.to_owned()).await {
        //         Ok(s) => {
        //             log!(
        //                 DEBUG,
        //                 "Successfully connected to endpoint, {}\n",
        //                 addr.endpoint
        //             );
        //             s
        //         }
        //         Err(err) => return HandleStatus::ConnectionFail(err.into()),
        //     };

        // match self.initiate_who_are_you(&mut stream).await {
        //     Ok(_) => (),
        //     Err(err) => return HandleStatus::WhoAreYouInitiateFail(err),
        // };

        // let way_ack = match self.receive_who_are_you_ack(stream).await {
        //     Ok(w) => w,
        //     Err(err) => return HandleStatus::WhoAreYouAckReceiveFail(err),
        // };

        // match self.handle_succeed_who_are_you(way_ack, addr).await {
        //     Ok(_) => (),
        //     Err(err) => return HandleStatus::PeerUpdateFail(err),
        // };

        HandleStatus::Success
    }
}
