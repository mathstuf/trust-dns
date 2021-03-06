use futures::future::*;

use trust_dns::client::*;
use trust_dns::rr::dnssec::*;
use trust_dns::rr::rdata::opt::EdnsOption;
use trust_dns::error::*;
use trust_dns::op::*;

#[derive(Clone)]
pub struct MutMessageClient<C: ClientHandle> {
    client: C,
    pub dnssec_ok: bool,
    pub support_algorithms: Option<SupportedAlgorithms>,
}

impl<C: ClientHandle> MutMessageClient<C> {
    pub fn new(client: C) -> Self {
        MutMessageClient {
            client,
            dnssec_ok: false,
            support_algorithms: None,
        }
    }
}

impl<C: ClientHandle> ClientHandle for MutMessageClient<C> {
    fn send(&mut self, mut message: Message) -> Box<Future<Item = Message, Error = ClientError>> {
        {
            // mutable block
            let edns = message.edns_mut();
            edns.set_dnssec_ok(true);

            if let Some(supported_algs) = self.support_algorithms {
                edns.set_option(EdnsOption::DAU(supported_algs));
            }
        }

        self.client.send(message)
    }
}