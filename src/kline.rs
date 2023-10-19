use serde_json::from_str;
use crate::mexc_structs;

#[derive(Clone)]
pub struct Kline {
    pub time: u64,
    pub open: u64,
    pub high: u64,
    pub low: u64,
    pub close: u64,
    pub volume: u64,
}

impl Kline {
    pub fn new(data: &mexc_structs::MexcKlineMessage) -> Self {
        Kline {
            time: data.d.k.t as u64,
            open: from_str::<u64>(&data.d.k.o).unwrap_or(0),
            high: from_str::<u64>(&data.d.k.h).unwrap_or(0),
            low: from_str::<u64>(&data.d.k.l).unwrap_or(0),
            close: from_str::<u64>(&data.d.k.c).unwrap_or(0),
            volume: from_str::<u64>(&data.d.k.v).unwrap_or(0),
        }
    }
}
