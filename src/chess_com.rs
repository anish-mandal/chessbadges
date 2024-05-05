use crate::badge::GameMode;
use reqwest;
use serde;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Last {
    pub rating: i32,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Game {
    pub last: Last,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ChessCom {
    pub chess_daily: Option<Game>,
    pub chess_blitz: Option<Game>,
    pub chess_bullet: Option<Game>,
    pub chess_rapid: Option<Game>,
}

pub async fn get_info(username: &str, mode: &GameMode) -> Result<String, reqwest::Error> {
    let req = reqwest::Client::new()
        .get(format!(
            "https://api.chess.com/pub/player/{}/stats",
            username
        ))
        .header(reqwest::header::USER_AGENT, "Yes")
        .send()
        .await?;

    if req.status() == reqwest::StatusCode::NOT_FOUND {
        return Ok("No player found".to_string());
    }

    let info = req.json::<ChessCom>().await?;

    match mode {
        GameMode::Rapid => match info.chess_rapid {
            Some(i) => Ok(i.last.rating.to_string()),
            None => Ok("No rapid games".to_string()),
        },
        GameMode::Blitz => match info.chess_blitz {
            Some(i) => Ok(i.last.rating.to_string()),
            None => Ok("No blitz games".to_string()),
        },
        GameMode::Daily => match info.chess_daily {
            Some(i) => Ok(i.last.rating.to_string()),
            None => Ok("No daily games".to_string()),
        },
        GameMode::Bullet => match info.chess_bullet {
            Some(i) => Ok(i.last.rating.to_string()),
            None => Ok("No bullet games".to_string()),
        },
    }
}
