use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct HighScoreDto {
    #[serde(rename = "userName")]
    pub user_name: String,
    pub score: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct HighScoreDocument {
    pub user_name: String,
    pub score: u8,
    pub timestamp: DateTime,
}

impl HighScoreDocument {
    const MAX_SCORE: u8 = 144;

    pub fn try_from_dto(dto: &HighScoreDto) -> Result<Self, String> {
        if dto.score > Self::MAX_SCORE {
            Err(format!("Invalid score {}: too high", dto.score))
        } else {
            Ok(HighScoreDocument {
                user_name: dto.user_name.clone(),
                score: dto.score,
                timestamp: DateTime::now(),
            })
        }
    }
}
