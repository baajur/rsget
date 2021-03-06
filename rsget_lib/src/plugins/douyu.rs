use crate::Streamable;
use std::time::{SystemTime, UNIX_EPOCH};

use regex::Regex;

use crate::utils::error::StreamError;

use crate::utils::downloaders::DownloadClient;
use chrono::prelude::*;

use stream_lib::StreamType;

use md5;

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
struct DouyuGgad {
    play4: String,
    play1: String,
    videop: String,
    play2: String,
    play5: String,
    play3: String,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
struct DouyuServer {
    ip: String,
    port: String,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
struct DouyuCdn {
    name: String,
    cdn: String,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
struct DouyuP2p {
    player: usize,
    use_p2p: usize,
    w_dm: usize,
    m_dm: usize,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
struct DouyuGift {
    stay_time: usize,
    pdhimg: String,
    gx: usize,
    mobile_big_effect_icon_0: String,
    mgif: String,
    cimg: String,
    mobile_big_effect_icon_1: String,
    big_efect_icon: String,
    pdbimg: String,
    mobile_stay_time: String,
    mimg: String,
    brgb: String,
    mobile_big_effect_icon_3: String,
    m_ef_gif_2: String,
    pimg: String,
    pt: String,
    id: String,
    intro: String,
    pc: String,
    m_ef_gif_1: String,
    urgb: String,
    ef: usize,
    sort: String,
    mobile_big_effect_icon_2: String,
    ch: String,
    effect: String,
    himg: String,
    #[serde(rename = "type")]
    adtype: String,
    gt: String,
    mobile_small_effect_icon: String,
    grgb: String,
    drgb: String,
    pad_big_effect_icon: String,
    desc: String,
    mobimg: String,
    small_effect_icon: String,
    name: String,
    mobile_icon_v2: String,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
struct DouyuMultiBR {
    middle: String,
    middle2: String,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
struct DouyuData {
    use_p2p: String,
    show_details: String,
    nickname: String,
    rtmp_url: String,
    ggad: DouyuGgad,
    anchor_city: String,
    specific_status: String,
    url: String,
    servers: Vec<DouyuServer>,
    rtmp_cdn: String,
    specific_catalog: String,
    cate_id1: usize,
    show_status: String,
    game_icon_url: String,
    game_name: String,
    cdnsWithName: Vec<DouyuCdn>,
    p2p_settings: DouyuP2p,
    show_time: String,
    isVertical: usize,
    rtmp_live: String,
    fans: String,
    game_url: String,
    room_src: String,
    is_white_list: String,
    room_name: String,
    owner_uid: String,
    owner_avatar: String,
    #[serde(skip_deserializing)]
    black: Vec<usize>, // Not sure about this one,
    vertical_src: String,
    room_dm_delay: usize,
    owner_weight: String,
    is_pass_player: usize,
    hls_url: String,
    room_id: usize,
    cur_credit: String,
    gift_ver: String,
    low_credit: String,
    #[serde(skip_deserializing)]
    gift: Vec<DouyuGift>,
    #[serde(skip_deserializing)]
    rtmp_multi_bitrate: String, //DouyuMultiBR, TODO: fix this it is broken at the moment
    cdns: Vec<String>,
    online: usize,
    credit_illegal: String,
    vod_quality: String,
    cate_id: String,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
struct DouyuRoom {
    error: usize,
    data: DouyuData,
}

#[derive(Clone, Debug)]
pub struct Douyu {
    data: DouyuRoom,
    room_id: u32,
    client: DownloadClient,
}

impl Streamable for Douyu {
    fn new(url: String) -> Result<Box<Douyu>, StreamError> {
        let dc = DownloadClient::new()?;
        let room_id_re = Regex::new(r"com/([a-zA-Z0-9]+)").unwrap();
        let cap = room_id_re.captures(&url).unwrap();

        let head = "Mozilla/5.0 (iPad; CPU OS 8_1_3 like Mac OS X) \
                    AppleWebKit/600.1.4 (KHTML, like Gecko) \
                    Version/8.0 Mobile/12B466 Safari/600.1.4";

        let room_id = match cap[1].parse::<u32>() {
            Ok(rid) => rid,
            Err(_) => {
                let re_room_id = Regex::new(r#""room_id" *:([0-9]+),"#).unwrap();
                let req = dc.make_request(&url, None)?;
                let body: String = dc.download_to_string(req)?;
                let cap = re_room_id.captures(&body).unwrap();
                cap[1].parse::<u32>().unwrap()
            }
        };

        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let ts = since_the_epoch.as_secs();

        let suffix = format!(
            "room/{}?aid=wp&cdn={}&client_sys=wp&time={}",
            &room_id, "ws", ts
        );

        let api_secret = b"zNzMV1y4EMxOHS6I5WKm";
        let mut hasher: md5::Context = md5::Context::new();

        hasher.consume(&suffix.as_bytes());
        hasher.consume(api_secret);

        let sign = format!("{:x}", hasher.compute());

        let json_url = format!("https://capi.douyucdn.cn/api/v1/{}&auth={}", &suffix, &sign);
        let json_req = dc.make_request(&json_url, Some(("User-Agent", head)))?;
        let jres: Result<DouyuRoom, StreamError> = dc.download_and_de::<DouyuRoom>(json_req);
        match jres {
            Ok(jre) => {
                let dy = Douyu {
                    data: jre,
                    room_id,
                    client: dc,
                };
                debug!("{:#?}", dy);
                Ok(Box::new(dy))
            }
            Err(why) => Err(why),
        }
    }

    fn get_title(&self) -> Option<String> {
        Some(String::from("test"))
    }

    fn get_author(&self) -> Option<String> {
        Some(self.data.data.nickname.clone())
    }

    fn is_online(&self) -> bool {
        self.data.data.online != 0
    }

    fn get_stream(&self) -> Result<StreamType, StreamError> {
        Ok(StreamType::Chuncked(
            self.client
                .rclient
                .get(&format!(
                    "{}/{}",
                    &self.data.data.rtmp_url, &self.data.data.rtmp_live
                ))
                .build()?,
        ))
    }

    fn get_ext(&self) -> String {
        String::from("flv")
    }

    fn get_default_name(&self) -> String {
        let local: DateTime<Local> = Local::now();
        format!(
            "{}-{:04}-{:02}-{:02}-{:02}-{:02}-{}-{}.{}",
            self.room_id,
            local.year(),
            local.month(),
            local.day(),
            local.hour(),
            local.minute(),
            self.get_author().unwrap(),
            self.get_title().unwrap_or_else(|| String::from("TEST")),
            self.get_ext()
        )
    }
    fn get_reqwest_client(&self) -> &reqwest::Client {
        &self.client.rclient
    }
}
