use actix_web::{
    get,
    middleware::Logger,
    web::{self, ServiceConfig},
    HttpResponse, Responder,
};
use badge::{ChessBadge, GameMode};
use serde::Deserialize;
use shuttle_actix_web::ShuttleActixWeb;

mod badge;
mod chess_com;

#[derive(Deserialize)]
struct Info {
    username: String,
    mode: GameMode,
}

#[get("/chess_com")]
async fn chesscom_provider(query: web::Query<Info>) -> impl Responder {
    let body = chess_com::get_info(&query.username, &query.mode).await;

    let badge;
    let is_parsable;

    match body {
        Ok(i) => {
            if i.parse::<i32>().is_ok() {
                badge = ChessBadge::new(Some(i), query.mode, None);
                is_parsable = true
            } else {
                badge = ChessBadge::new(None, query.mode, Some(i));
                is_parsable = false
            }
        }
        Err(e) => {
            badge = ChessBadge::new(None, query.mode, Some(e.to_string()));
            return HttpResponse::InternalServerError()
                .content_type("image/svg+xml")
                .insert_header(("Cache-Control", "max-age=500"))
                .body(badge.error());
        }
    };

    HttpResponse::Ok()
        .content_type("image/svg+xml")
        .insert_header(("Cache-Control", "max-age=500"))
        .body(if is_parsable {
            badge.render()
        } else {
            badge.error()
        })
}

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(
            web::scope("")
                .wrap(Logger::default())
                .service(chesscom_provider),
        );
    };

    Ok(config.into())
}
