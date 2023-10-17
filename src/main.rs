use url::Url;
use tungstenite::{connect, Message, WebSocket};
use serde_json::{from_str, json};
use std::collections::HashMap;
use std::net::TcpStream;
use tungstenite::stream::MaybeTlsStream;

mod websocket_structs;

#[derive(Clone)]
struct Kline {
    pub time: u64,
    pub open: u64,
    pub high: u64,
    pub low: u64,
    pub close: u64,
    pub volume: u64,
}

const MAX_MEXC_WS_SUBSCRIPTION: usize = 30;

#[tokio::main]
async fn main() {
    let mut klines: HashMap<String, Vec<Kline>> = HashMap::new();

    let exchange_info = reqwest::get("https://api.mexc.com/api/v3/exchangeInfo")
        .await.unwrap()
        .json::<websocket_structs::MexcExchangeInfo>()
        .await.unwrap();

    let mut params: Vec<String> = vec![];
    let mut sockets: Vec<WebSocket<MaybeTlsStream<TcpStream>>> = vec![];

    for i in exchange_info.symbols {
        params.push(i.symbol);

        if params.len() >= MAX_MEXC_WS_SUBSCRIPTION {
            let socket = create_socket(&params);
            sockets.push(socket);
            params.clear();
        }
    }
    println!("{}", sockets.len());
    ws_event_loop(&mut sockets, &mut klines);
}


fn insert_kline_in_pool(klines: &mut Vec<Kline>, data: &websocket_structs::MexcKlineMessage) {
    let kline = Kline {
        time: data.d.k.t as u64,
        open: from_str::<u64>(&data.d.k.o).unwrap_or(0),
        high: from_str::<u64>(&data.d.k.h).unwrap_or(0),
        low: from_str::<u64>(&data.d.k.l).unwrap_or(0),
        close: from_str::<u64>(&data.d.k.c).unwrap_or(0),
        volume: from_str::<u64>(&data.d.k.v).unwrap_or(0),
    };

    let last_index = klines.len() - 1;
    let result = klines.get(last_index)
        .unwrap_or_else(|| {
            klines.push(kline.clone());

            klines.last().unwrap()
        });

    if result.time != kline.time {
        klines.push(kline.clone());
    } else {
        klines.insert(last_index, kline.clone());
    }
}

fn ws_event_loop(sockets: &mut Vec<WebSocket<MaybeTlsStream<TcpStream>>>, klines: &mut HashMap<String, Vec<Kline>>) {
    loop {
        for socket in &mut *sockets {
            let msg = socket.read().unwrap().to_string();

            if let Ok(json) = from_str::<websocket_structs::MexcKlineMessage>(&msg) {
                let symbol = json.s.clone();
                let candles = klines.get_mut(&symbol)
                    .unwrap_or_else(|| {
                        let new_vec: Vec<Kline> = vec![];

                        return &mut klines.insert(symbol, new_vec.clone()).unwrap();
                    });

                insert_kline_in_pool(candles, &json);

                println!("Symbol: {}, Close: {}", json.s, json.d.k.c);
            }

            if let Ok(subscribe_msg) = serde_json::from_str::<websocket_structs::MexcSubscribeResponseMessage>(&msg) {
                println!("Subscribe {}", subscribe_msg.msg);
                continue;
            }
        }
    }
}

fn create_socket(symbols: &Vec<String>) -> WebSocket<MaybeTlsStream<TcpStream>> {
    let mut socket = create_ws_connection();

    for i in (0..symbols.len()).step_by(10) {
        let params = symbols[i..(i + 10)]
            .iter()
            .map(|e| String::from(format!("spot@public.kline.v3.api@{}@Min1", e)))
            .collect();

        let subscribe_data = websocket_structs::MexcSubscribeMessage {
            method: String::from("SUBSCRIPTION"),
            params,
        };

        let subscribe_json = json!(subscribe_data).to_string();

        socket.send(Message::Text(subscribe_json)).unwrap();
    }

    return socket;
}

fn create_ws_connection() -> WebSocket<MaybeTlsStream<TcpStream>> {
    let (socket, _response) = connect(
        Url::parse("wss://wbs.mexc.com/ws").unwrap()
    ).expect("Can't connect");

    return socket;
}