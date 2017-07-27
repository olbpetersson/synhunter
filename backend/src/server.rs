
use jsonrpc_core::{Error, Metadata, MetaIoHandler};
use jsonrpc_core::futures::{BoxFuture, Future, future};
use jsonrpc_macros::pubsub;
use jsonrpc_pubsub::{PubSubHandler, PubSubMetadata, Session, SubscriptionId};
use jsonrpc_ws_server::{self, MetaExtractor, Server, ServerBuilder};

use std::net::{SocketAddr, Ipv4Addr, IpAddr};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::str::FromStr;

use structs::{Game, ClientState};
use wordlist::Wordlist;
use uuid::Uuid;
use rand::{self, Rng};

const WORDLIST: &str = "../words/saoldict.txt";
const EASY_WORDLIST: &str = "../words/easy_words.txt";

build_rpc_trait! {
    pub trait GameApi {
        type Metadata;

        #[rpc(meta, name = "foo")]
        fn foo(&self, Self::Metadata, i64) -> BoxFuture<i64, Error>;

        #[rpc(meta, name = "reset_game")]
        fn reset_game(&self, Self::Metadata) -> BoxFuture<(), Error>;

        #[rpc(meta, name = "choose_tile")]
        fn choose_tile(&self, Self::Metadata, Uuid) -> BoxFuture<(), Error>;

        #[rpc(meta, name = "submit_hint")]
        fn submit_hint(&self, Self::Metadata, String) -> BoxFuture<(), Error>;

        #[rpc(meta, name = "submit_answer")]
        fn submit_answer(&self, Self::Metadata, String) -> BoxFuture<(), Error>;

        #[pubsub(name = "game")] {
            #[rpc(name = "game_subscribe")]
            fn game_subscribe(&self, Self::Metadata, pubsub::Subscriber<ClientState>);

            #[rpc(name = "game_unsubscribe")]
            fn game_unsubscribe(&self, SubscriptionId) -> BoxFuture<(), Error>;
        }
    }
}

pub struct GameServer {
    state: Mutex<GameServerState>,
}

impl GameServer {
    pub fn new() -> GameServer {
        GameServer {
            state: Mutex::new(GameServerState::new()),
        }
    }
}

struct GameServerState {
    game: Game,
    subscribers: HashMap<Uuid, pubsub::Sink<ClientState>>,
    wordlist: Wordlist,
    easy_wordlist: Wordlist,
}

impl GameServerState {
    fn new() -> GameServerState {
        let wordlist = Wordlist::new(WORDLIST);
        let easy_wordlist = Wordlist::new(EASY_WORDLIST);
        GameServerState {
            game: Game::new(&easy_wordlist),
            subscribers: HashMap::new(),
            wordlist,
            easy_wordlist,
        }
    }

    fn broadcast_state(&mut self) {
        println!("get_client_state: {:#?}", self.game.turns.last().unwrap());
        for sink in self.subscribers.values() {
            let _ = sink.notify(Ok(self.get_client_state())).wait();
        }
    }

    fn get_client_state(&self) -> ClientState {
        ClientState {
            board: self.game.board.clone(),
            turns: self.game.turns.clone(),
        }
    }

    fn reset_game(&mut self) {
        self.game = Game::new(&self.easy_wordlist);
        let mut players: Vec<Uuid> = self.subscribers.keys().cloned().collect();
        let mut rng = rand::thread_rng();
        while !players.is_empty() {
            let i = rng.gen::<usize>() % players.len();
            self.game.board.add_player(players.remove(i));
        }
    }
}

impl GameApi for GameServer {
    type Metadata = Meta;

    fn foo(&self, meta: Self::Metadata, i: i64) -> BoxFuture<i64, Error> {
        println!("Hello foo: {}: {}", i, meta.id.to_string());
        future::ok(i+100).boxed()
    }

    fn reset_game(&self, _meta: Self::Metadata) -> BoxFuture<(), Error> {
        let mut state = self.state.lock().unwrap();
        state.reset_game();
        state.broadcast_state();
        future::ok(()).boxed()
    }

    fn choose_tile(&self, meta: Self::Metadata, tile: Uuid) -> BoxFuture<(), Error> {
        let mut state = self.state.lock().unwrap();
        state.game.choose_tile(meta.id, tile);
        state.broadcast_state();
        future::ok(()).boxed()
    }

    fn submit_hint(&self, meta: Self::Metadata, hint: String) -> BoxFuture<(), Error> {
        let hint = hint.trim().to_lowercase();
        let mut state = self.state.lock().unwrap();
        if !state.wordlist.has_word(&hint) {
            println!("Hint not in wordlist");
        } else {
            state.game.submit_hint(meta.id, hint);
        }
        state.broadcast_state();
        future::ok(()).boxed()
    }

    fn submit_answer(&self, meta: Self::Metadata, answer: String) -> BoxFuture<(), Error> {
        let answer = answer.trim().to_lowercase();
        let mut state = self.state.lock().unwrap();
        state.game.submit_answer(meta.id, answer);
        state.broadcast_state();
        future::ok(()).boxed()
    }

    fn game_subscribe(&self, meta: Self::Metadata, subscriber: pubsub::Subscriber<ClientState>) {
        let mut state = self.state.lock().unwrap();
        
        let uuid_string = meta.id.to_string();
        let id = SubscriptionId::String(uuid_string.clone());
        let sink = subscriber.assign_id(id.clone()).unwrap();

        state.game.board.add_player(meta.id);
        
        state.subscribers.insert(meta.id, sink);

        state.broadcast_state();
    }

    fn game_unsubscribe(&self, id: SubscriptionId) -> BoxFuture<(), Error> {
        let uuid = if let SubscriptionId::String(uuid_string) = id {
            Uuid::from_str(&uuid_string).unwrap()
        } else {
            panic!("FAIL")
        };

        let mut state = self.state.lock().unwrap();
        state.subscribers.remove(&uuid);

        if state.game.board.remove_player(uuid) {
            state.reset_game();
        }
        state.broadcast_state();
        future::ok(()).boxed()
    }
}

pub fn create_server() -> RpcServer {
    let game_server = GameServer::new();
    let mut io = PubSubHandler::default();
    io.extend_with(game_server.to_delegate());

    RpcServer::start_with_metadata(io.into(), meta_extractor).unwrap()
}


pub struct RpcServer {
    server: Server,
}

impl RpcServer {
    pub fn start_with_metadata<M, E>(handler: MetaIoHandler<M>, meta_extractor: E) -> Result<Self, String>
        where M: Metadata,
              E: MetaExtractor<M>
    {
        let listen_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 1337);
        ServerBuilder::new(handler)
            .session_meta_extractor(meta_extractor)
            .start(&listen_addr)
            .map(
                |server| {
                    RpcServer {
                        server: server,
                    }
                },
            ).map_err(|e| e.to_string())
    }

    /// Consumes the server and waits for it to finish. Get an `CloseHandle` before calling this
    /// if you want to be able to shut the server down.
    pub fn wait(self) -> Result<(), String> {
        self.server.wait().map_err(|e| e.to_string())
    }
}



/// The metadata type. There is one instance associated with each connection. In this pubsub
/// scenario they are created by `meta_extractor` by the server on each new incoming
/// connection.
#[derive(Clone, Debug, Default)]
pub struct Meta {
    session: Option<Arc<Session>>,
    id: Uuid,
}

/// Make the `Meta` type possible to use as jsonrpc metadata type.
impl Metadata for Meta {}

/// Make the `Meta` type possible to use as a pubsub metadata type.
impl PubSubMetadata for Meta {
    fn session(&self) -> Option<Arc<Session>> {
        self.session.clone()
    }
}

/// Metadata extractor function for `Meta`.
fn meta_extractor(context: &jsonrpc_ws_server::RequestContext) -> Meta {
    Meta {
        session: Some(Arc::new(Session::new(context.sender()))),
        id: Uuid::new_v4(),
    }
}
