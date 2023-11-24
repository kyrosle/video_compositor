use anyhow::Result;
use compositor_common::scene::Resolution;
use log::{error, info};
use serde_json::json;
use std::{
    process::{Command, Stdio},
    thread,
    time::Duration,
};
use video_compositor::{api::Response, http};

use crate::common::write_example_sdp_file;

#[path = "./common/common.rs"]
mod common;

const VIDEO_RESOLUTION: Resolution = Resolution {
    width: 1920,
    height: 1080,
};
const FRAMERATE: u32 = 30;

fn main() {
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );
    ffmpeg_next::format::network::init();

    thread::spawn(|| {
        if let Err(err) = start_example_client_code() {
            error!("{err}")
        }
    });

    http::Server::new(8001).run();
}

fn start_example_client_code() -> Result<()> {
    thread::sleep(Duration::from_secs(2));

    info!("[example] Sending init request.");
    common::post(&json!({
        "type": "init",
        "web_renderer": {
            "init": false
        },
        "framerate": FRAMERATE,
        "stream_fallback_timeout_ms": 2000
    }))?;

    info!("[example] Start listening on output port.");
    let output_sdp = write_example_sdp_file("127.0.0.1", 8002)?;
    Command::new("ffplay")
        .args(["-protocol_whitelist", "file,rtp,udp", &output_sdp])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;

    info!("[example] Send register output request.");
    common::post(&json!({
        "type": "register",
        "entity_type": "output_stream",
        "output_id": "output_1",
        "port": 8002,
        "ip": "127.0.0.1",
        "resolution": {
            "width": VIDEO_RESOLUTION.width,
            "height": VIDEO_RESOLUTION.height,
        },
        "encoder_settings": {
            "preset": "ultrafast"
        }
    }))?;

    info!("[example] Send register input request.");
    let response = common::post(&json!({
        "type": "register",
        "entity_type": "input_stream",
        "input_id": "input_1",
        "port": "8004:8008"
    }))?
    .json()?;

    let input_port = match response {
        Response::RegisteredPort(port) => port,
        _ => panic!(
            "Unexpected response for registering an input: {:?}",
            response
        ),
    };

    info!("[example] Register static images");
    common::post(&json!({
        "type": "register",
        "entity_type": "image",
        "image_id": "example_gif",
        "asset_type": "gif",
        "url": "https://gifdb.com/images/high/rust-logo-on-fire-o41c0v9om8drr8dv.gif",
    }))?;

    let shader_source = include_str!("./silly.wgsl");
    info!("[example] Register shader transform");
    common::post(&json!({
        "type": "register",
        "entity_type": "shader",
        "shader_id": "example_shader",
        "source": shader_source,
        "fallback_strategy": "fallback_if_all_inputs_missing",
    }))?;

    info!("[example] Start pipeline");
    common::post(&json!({
        "type": "start",
    }))?;

    info!("[example] Start input stream");
    let ffmpeg_source = format!(
        "testsrc=s={}x{}:r=30,format=yuv420p",
        VIDEO_RESOLUTION.width, VIDEO_RESOLUTION.height
    );
    Command::new("ffmpeg")
        .args([
            "-re",
            "-f",
            "lavfi",
            "-i",
            &ffmpeg_source,
            "-c:v",
            "libx264",
            "-f",
            "rtp",
            &format!("rtp://127.0.0.1:{}?rtcpport={}", input_port, input_port),
        ])
        .spawn()?;
    thread::sleep(Duration::from_secs(5));

    let input_with_shader = json!( {
        "type": "shader",
        "id": "shader_1",
        "shader_id": "example_shader",
        "children": [
            {
                "type": "view",
                "width": 1080,
                "height": 1080,
                "children": [
                    {
                        "type": "image",
                        "image_id": "example_gif",
                    }
                ]
            }
        ],
        "resolution": { "width": VIDEO_RESOLUTION.width / 5, "height": VIDEO_RESOLUTION.height / 5 },
    });

    let text = json!({
        "type": "text",
        "text": "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nunc facilisis faucibus lacus et ornare. Vestibulum sed felis id metus maximus consectetur a eget nisi. Nullam in fringilla ipsum. Morbi faucibus eget enim vel pellentesque. Nullam semper ex nec nibh fringilla dapibus.",
        "width": 400,
        "font_size": 20,
        "wrap": "word",
        "color_rgba": "#FFFFFFFF"
    });

    let layout = json!({
        "type": "view",
        "children": [
            {
                "id": "test_1",
                "type": "view",
                "background_color_rgba": "#FF0000FF",
                "width": 200,
                "children": [
                    {
                        "type": "view",
                        "background_color_rgba": "#0000FFFF",
                        "height": 500,
                        "children": [text]
                    },
                    {
                        "type": "view",
                        "background_color_rgba": "#00FFFFFF",
                        "height": 500,
                        "width": 100,
                    }
                ]
            },
            {
                "type": "view",
                "background_color_rgba": "#00FF00FF",
                "children": [
                    input_with_shader,
                ]
            },
        ]
    });

    info!("[example] Update scene");
    common::post(&json!({
        "type": "update_scene",
        "scenes": [
            {
                "output_id": "output_1",
                "root": layout,
            }
        ],
    }))?;
    thread::sleep(Duration::from_secs(5));

    let layout_2 = json!({
        "type": "view",
        "children": [
            {
                "id": "test_1",
                "type": "view",
                "background_color_rgba": "#FF0000FF",
                "width": 800,
                "transition": {
                    "duration_ms": 10000
                },
                "children": [
                    {
                        "type": "view",
                        "background_color_rgba": "#0000FFFF",
                        "height": 500,
                        "children": [text]
                    },
                    {
                        "type": "view",
                        "background_color_rgba": "#00FFFFFF",
                        "height": 500,
                        "width": 100,
                    }
                ]
            },
            {
                "type": "view",
                "background_color_rgba": "#00FF00FF",
                "children": [
                    input_with_shader,
                ]
            },
        ]
    });

    info!("[example] Update scene");
    common::post(&json!({
        "type": "update_scene",
        "scenes": [
            {
                "output_id": "output_1",
                "root": layout_2,
            }
        ],
    }))?;
    Ok(())
}
