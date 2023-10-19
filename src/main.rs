use url::Url;
use tungstenite::{connect, Message, WebSocket};
use serde_json::{from_str, json};
use std::collections::HashMap;
use std::net::TcpStream;
use std::string::ToString;
use tungstenite::stream::MaybeTlsStream;
use querystring::{QueryParams, stringify as qs_stringify};

use kline::Kline;
use mexc_structs::{MexcExchangeInfo, MexcKlineMessage, MexcSubscribeMessage, MexcKlineResponse, MexcSubscribeResponseMessage};

mod mexc_structs;
mod kline;

const MAX_MEXC_WS_SUBSCRIPTION: usize = 30;

#[tokio::main]
async fn main() {
    let mut klines: HashMap<String, Vec<Kline>> = HashMap::new();

    let exchange_info = reqwest::get("https://api.mexc.com/api/v3/exchangeInfo")
        .await.unwrap()
        .json::<MexcExchangeInfo>()
        .await.unwrap();

    let mut params: Vec<String> = vec![];
    let mut sockets: Vec<WebSocket<MaybeTlsStream<TcpStream>>> = vec![];
    let mut counter = 0;

    for i in &exchange_info.symbols[..30] {
        let symbol = i.symbol.clone();
        let klines_result = get_all_candles(&symbol, String::from("15m")).await;

        klines.insert(symbol, klines_result);

        params.push(i.symbol.clone());
        counter += 1;

        println!("{} | {}/{} subscriptions", i.symbol, counter, &exchange_info.symbols.len());

        if params.len() >= MAX_MEXC_WS_SUBSCRIPTION {
            let socket = create_socket(&params);
            sockets.push(socket);
            params.clear();
        }
    }

    ws_event_loop(&mut sockets, &mut klines);
}

async fn get_all_candles(symbol: &str, interval: String) -> Vec<Kline> {
    let mut klines: Vec<Kline> = vec![];
    let mut stop = false;
    let mut end_time: i64 = 0;

    while !stop {
        let start_time_str = (end_time - 900000000).to_string();
        let end_time_str = end_time.to_string();

        let mut params: QueryParams = vec![
            ("symbol", symbol),
            ("interval", &interval),
            ("limit", "1000"),
            ("startTime", &start_time_str),
            ("endTime", &end_time_str),
        ];

        if end_time == 0 {
            params.pop();
            params.pop();
        }

        let klines_result = reqwest::get(format!("https://api.mexc.com/api/v3/klines?{}", qs_stringify(params)))
            .await.unwrap()
            .json::<MexcKlineResponse>()
            .await.unwrap();

        let mut parsed_klines: Vec<Kline> = klines_result.iter().map(|data| {
            Kline {
                time: data.0,
                open: from_str::<u64>(&data.1).unwrap_or(0),
                high: from_str::<u64>(&data.2).unwrap_or(0),
                low: from_str::<u64>(&data.3).unwrap_or(0),
                close: from_str::<u64>(&data.4).unwrap_or(0),
                volume: from_str::<u64>(&data.5).unwrap_or(0),
            }
        }).collect();

        if end_time != 0 {
            parsed_klines.remove(0);
        }

        klines = [parsed_klines, klines].concat();

        if klines_result.len() < 1000 {
            stop = true;
        } else {
            end_time = klines.first().unwrap().time as i64;
        }
    }

    return klines;
}

fn insert_kline_in_pool(klines: &mut Vec<Kline>, data: &MexcKlineMessage) {
    if klines.len() == 0 {
        klines.push(Kline::new(&data));
    }

    let result = klines.last().unwrap();
    let kline = Kline::new(&data);

    if result.time != kline.time {
        klines.push(kline);
    } else {
        let _ = klines.last().insert(&kline);
    }
}

fn ws_event_loop(sockets: &mut Vec<WebSocket<MaybeTlsStream<TcpStream>>>, klines: &mut HashMap<String, Vec<Kline>>) {
    loop {
        for socket in &mut *sockets {
            let msg = socket.read().unwrap().to_string();

            if let Ok(json) = from_str::<MexcKlineMessage>(&msg) {
                println!("Symbol: {}, Close: {}", &json.s, &json.d.k.c);

                let candles = klines.entry(json.s.clone()).or_insert(vec![]);

                insert_kline_in_pool(candles, &json);

                continue;
            }

            if let Ok(_subscribe_msg) = from_str::<MexcSubscribeResponseMessage>(&msg) {
                // println!("Subscribe {}", subscribe_msg.msg);
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

        let subscribe_data = MexcSubscribeMessage {
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