use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use badge::{ChessBadge, GameMode};
use serde::Deserialize;

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(chesscom_provider))
        .bind(("127.0.0.1", 3000))?
        .run()
        .await
}
