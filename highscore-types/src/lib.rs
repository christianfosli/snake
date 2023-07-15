use bson::oid::ObjectId;
use bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct HighScoreDto {
    #[serde(rename = "userName")]
    pub user_name: String,
    pub score: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct HighScoreDocument {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
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
                id: None,
                user_name: dto.user_name.clone(),
                score: dto.score,
                timestamp: DateTime::now(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_maps_from_dto_to_doc() {
        let dto = HighScoreDto {
            user_name: String::from("Test user"),
            score: 50,
        };
        let doc = HighScoreDocument::try_from_dto(&dto).unwrap();

        assert_eq!(doc.user_name, dto.user_name);
        assert_eq!(doc.score, dto.score);

        // timestamp should be ~= now
        assert!(
            doc.timestamp <= DateTime::now()
                && doc.timestamp > DateTime::from_millis(DateTime::now().timestamp_millis() - 500)
        );
    }
}
