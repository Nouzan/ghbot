// The version of Heroku ping-pong-bot, which uses a webhook to receive updates
// from Telegram, instead of long polling.

use teloxide::{dispatching::update_listeners, prelude::*, types::ParseMode};

use ghbot::github::Payload;
use reqwest::StatusCode;
use std::{convert::Infallible, env, net::SocketAddr};
use tokio::sync::mpsc;
use warp::Filter;

#[tokio::main]
async fn main() {
    run().await;
}

async fn handle_rejection(error: warp::Rejection) -> Result<impl warp::Reply, Infallible> {
    log::error!("Cannot process the request due to: {:?}", error);
    Ok(StatusCode::INTERNAL_SERVER_ERROR)
}

async fn handle_gh(
    state: (Bot, i64, String, Payload),
) -> Result<impl warp::Reply, warp::Rejection> {
    let (bot, chat, event, payload) = state;
    let message = match payload {
        Payload::IssueEvent(payload) => format!(
            "Issue[#{}]({}) {}",
            payload.issue.number, payload.issue.url, payload.action
        ),
        Payload::Common(payload) => format!("Event: `{}` {}", event, payload.action),
    };
    log::debug!("ready to send: {}", message);
    if let Err(error) = bot
        .send_message(chat, message)
        .parse_mode(ParseMode::MarkdownV2)
        .send()
        .await
    {
        log::error!("send message error: {}", error);
        Err(warp::reject::reject())
    } else {
        Ok(warp::reply())
    }
}

pub async fn webhook<'a>(bot: Bot) -> impl update_listeners::UpdateListener<Infallible> {
    // Heroku defines auto defines a port value
    let teloxide_token = env::var("TELOXIDE_TOKEN").expect("TELOXIDE_TOKEN env variable missing");
    let port: u16 = env::var("PORT")
        .expect("PORT env variable missing")
        .parse()
        .expect("PORT value to be integer");
    // Heroku host example .: "heroku-ping-pong-bot.herokuapp.com"
    let host = env::var("HOST").expect("have HOST env variable");
    let path = format!("bot{}", teloxide_token);
    let url = format!("https://{}/{}", host, path);
    let chat: i64 = env::var("CHAT")
        .expect("CHAT env variable missing")
        .parse()
        .expect("CHAT value to be integer");

    bot.set_webhook(url)
        .send()
        .await
        .expect("Cannot setup a webhook");

    let (tx, rx) = mpsc::unbounded_channel();

    let tg = warp::post()
        .and(warp::path(path))
        .and(warp::body::json())
        .map(move |json: serde_json::Value| {
            let try_parse = match serde_json::from_str(&json.to_string()) {
                Ok(update) => Ok(update),
                Err(error) => {
                    log::error!(
                        "Cannot parse an update.\nError: {:?}\nValue: {}\n\
                       This is a bug in teloxide, please open an issue here: \
                       https://github.com/teloxide/teloxide/issues.",
                        error,
                        json
                    );
                    Err(error)
                }
            };
            if let Ok(update) = try_parse {
                tx.send(Ok(update))
                    .expect("Cannot send an incoming update from the webhook")
            }

            StatusCode::OK
        });

    let gh = warp::path!("gh")
        .and(warp::post())
        .and(warp::header("X-GitHub-Event"))
        .and(warp::body::json())
        .map(move |event: String, payload: serde_json::Value| {
            log::debug!("gh: {}", serde_json::to_string(&payload).unwrap());
            (
                bot.clone(),
                chat,
                event,
                serde_json::from_value(payload).unwrap(),
            )
        })
        .and_then(handle_gh);

    let server = tg.or(gh).recover(handle_rejection);

    let serve = warp::serve(server);

    let address = format!("0.0.0.0:{}", port);
    tokio::spawn(serve.run(address.parse::<SocketAddr>().unwrap()));
    rx
}

async fn run() {
    teloxide::enable_logging!();
    log::info!("Starting heroku_ping_pong_bot...");
    let bot = Bot::from_env();

    let cloned_bot = bot.clone();
    teloxide::repl_with_listener(
        bot,
        |message| async move {
            message
                .answer_str(format!("chat: {}", message.chat_id()))
                .await?;
            ResponseResult::<()>::Ok(())
        },
        webhook(cloned_bot).await,
    )
    .await;
}
