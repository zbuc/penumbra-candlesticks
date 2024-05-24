use proto::penumbra::core::asset::v1::AssetId;
use proto::penumbra::core::component::dex::v1::query_service_client::QueryServiceClient;
use proto::penumbra::core::component::dex::v1::{CandlestickDataRequest, DirectedTradingPair};

use cli_candlestick_chart::{Candle, Chart};

pub mod proto {
    pub mod penumbra {
        /// Core protocol structures.
        pub mod core {
            pub mod asset {
                pub mod v1 {
                    include!("../gen/proto/penumbra.core.asset.v1.rs");
                }
            }
            pub mod component {
                pub mod dex {
                    pub mod v1 {
                        include!("../gen/proto/penumbra.core.component.dex.v1.rs");
                    }
                }
                pub mod sct {
                    pub mod v1 {
                        include!("../gen/proto/penumbra.core.component.sct.v1.rs");
                    }
                }
                pub mod fee {
                    pub mod v1 {
                        include!("../gen/proto/penumbra.core.component.fee.v1.rs");
                    }
                }
                pub mod shielded_pool {
                    pub mod v1 {
                        include!("../gen/proto/penumbra.core.component.shielded_pool.v1.rs");
                    }
                }
            }
            pub mod keys {
                pub mod v1 {
                    include!("../gen/proto/penumbra.core.keys.v1.rs");
                }
            }
            pub mod num {
                pub mod v1 {
                    include!("../gen/proto/penumbra.core.num.v1.rs");
                }
            }
            pub mod txhash {
                pub mod v1 {
                    include!("../gen/proto/penumbra.core.txhash.v1.rs");
                }
            }
        }

        pub mod crypto {
            pub mod decaf377_rdsa {
                pub mod v1 {
                    include!("../gen/proto/penumbra.crypto.decaf377_rdsa.v1.rs");
                }
            }

            pub mod tct {
                pub mod v1 {
                    include!("../gen/proto/penumbra.crypto.tct.v1.rs");
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = QueryServiceClient::connect("http://localhost:8080").await?;

    let request = tonic::Request::new(CandlestickDataRequest {
        pair: Some(DirectedTradingPair {
            start: Some(AssetId {
                alt_base_denom: "upenumbra".to_string(),
                inner: vec![],
                alt_bech32m: "".to_string(),
            }),
            end: Some(AssetId {
                alt_base_denom: "ugm".to_string(),
                inner: vec![],
                alt_bech32m: "".to_string(),
            }),
        }),
        limit: 20_000,
        start_height: 0,
    });

    let response = client.candlestick_data(request).await?;

    println!("RESPONSE={:?}", response);

    let candles = response
        .into_inner()
        .data
        .into_iter()
        .map(|candle| {
            let timestamp: i64 = candle.height.try_into().unwrap();
            let timestamp = timestamp * 5i64;
            Candle {
                open: candle.open,
                high: candle.high,
                low: candle.low,
                close: candle.close,
                volume: Some(candle.direct_volume),
                // TODO: convert to timestamp
                timestamp: Some(timestamp),
            }
        })
        .collect::<Vec<_>>();
    // pub struct Candle {
    //     pub open: f64,
    //     pub high: f64,
    //     pub low: f64,
    //     pub close: f64,
    //     pub volume: Option<f64>,
    //     pub timestamp: Option<i64>,
    // }
    // Create and display the chart
    let mut chart = Chart::new(&candles);

    // Set the chart title
    chart.set_name(String::from("upenumbra/ugm"));

    // Set customs colors
    chart.set_bear_color(255, 0, 0);
    chart.set_bull_color(1, 255, 1);
    chart.set_vol_bull_color(1, 205, 254);
    chart.set_vol_bear_color(255, 107, 153);

    chart.set_volume_pane_height(6);
    chart.set_volume_pane_enabled(true);

    chart.draw();

    Ok(())
}
