#[macro_export]
macro_rules! mock_response {
    ($mod:literal, $fn:literal) => {{

    }};
    ($client:expr, $mod:literal, $fn:literal, $cfg:expr) => {{
        use std::fs::read_to_string;
        let cfg = format!("{:#?}", $cfg).replace(": ", "-").replace("\n", "").replace(" ", "").replace(",)", ")").replace(",}", "}").replace("\"", "");
        #[cfg(feature = "auth")]
        let mut filename = format!("./tests/files/{mod}/{fn}/{cfg}.json", mod=$mod, fn=$fn, cfg=cfg);
        #[cfg(not(feature = "auth"))]
        let filename = format!("./tests/files/{mod}/{fn}/{cfg}.json", mod=$mod, fn=$fn, cfg=cfg);
        dbg!(&filename);
        #[cfg(feature = "auth")]
        if $client.client.default_headers.get("Authorization").unwrap() == &format!("token {}", crate::constants::BAD_FAKE_TOKEN).parse::<::reqwest::header::HeaderValue>().unwrap() {
            filename = "./tests/files/Bad_creds.json".to_owned();
        }
        read_to_string(filename).unwrap()
    }};
}
