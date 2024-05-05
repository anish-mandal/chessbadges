use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum GameMode {
    Rapid,
    Blitz,
    Bullet,
    Daily,
}

impl GameMode {
    pub fn bg(&self) -> &str {
        match self {
            Self::Rapid => "#79AC47",
            Self::Blitz => "#FAD541",
            Self::Bullet => "#DCA623",
            Self::Daily => "#F7C631",
        }
    }
}

pub struct ChessBadge {
    pub rating: Option<String>,
    pub mode: GameMode,
    pub error_msg: Option<String>,
}

impl ChessBadge {
    pub fn new(rating: Option<String>, mode: GameMode, error_msg: Option<String>) -> ChessBadge {
        ChessBadge {
            rating,
            mode,
            error_msg,
        }
    }

    pub fn render(&self) -> String {
        match &self.rating {
            Some(i) => {
                let x = 6 + get_width(&format!("{:?}", self.mode));
                let total_width = get_width(&i) + x - 2;

                format!(
                    r##"
        <svg width="{}" height="20" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink">
            <rect x="0" y="0" width="100%" height="100%" fill="{}" rx="5" />
            <g fill="white" font-family="system-ui, sans-serif" transform="scale(0.9)" font-size="15">
                <text x="5" y="80%">{:?}</text>
                <text x="{}" fill="white" y="80%">{}</text>
            </g>
        </svg>"##,
                    total_width,
                    self.mode.bg(),
                    self.mode,
                    x,
                    i
                )
            }
            None => self.error(),
        }
    }

    pub fn error(&self) -> String {
        match &self.error_msg {
            Some(i) => {
                let total_width = get_width(&i);

                format!(
                    r##"
        <svg width="{}" height="20" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink">
            <rect x="0" y="0" width="100%" height="100%" fill="{}" rx="5" />
            <g fill="white" font-family="system-ui, sans-serif" transform="scale(0.9)" font-size="15">
                <text x="5" y="80%">{:?}</text>
            </g>
        </svg>"##,
                    total_width,
                    self.mode.bg(),
                    i
                )
            }
            None => " ".to_string(),
        }
    }
}

pub fn get_width(text: &String) -> usize {
    text.len() * 8
}
