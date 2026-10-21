use std::cmp::Ordering;

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub enum Difficulty {
    #[default]
    Easy,
    Medium,
    Hard,
}

impl PartialOrd for Difficulty {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Difficulty {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Difficulty::Easy, Difficulty::Easy) => Ordering::Equal,
            (Difficulty::Easy, _) => Ordering::Less,
            (Difficulty::Medium, Difficulty::Easy) => Ordering::Greater,
            (Difficulty::Medium, Difficulty::Medium) => Ordering::Equal,
            (Difficulty::Medium, Difficulty::Hard) => Ordering::Less,
            (Difficulty::Hard, Difficulty::Hard) => Ordering::Equal,
            (Difficulty::Hard, _) => Ordering::Greater,
        }
    }
}

#[derive(Default, PartialEq, Eq)]
pub enum SortBy {
    Difficulty,
    LikesDislikesRatio,
    #[default]
    Id,
    AcceptanceRate,
}

use std::fmt;

use serde::{Deserialize, Serialize};

impl fmt::Display for Difficulty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Difficulty::Easy => write!(f, "Easy"),
            Difficulty::Medium => write!(f, "Medium"),
            Difficulty::Hard => write!(f, "Hard"),
        }
    }
}

impl fmt::Display for SortBy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SortBy::Difficulty => write!(f, "Difficulty"),
            SortBy::LikesDislikesRatio => write!(f, "Likes Dislikes Ratio"),
            SortBy::Id => write!(f, "Id"),
            SortBy::AcceptanceRate => write!(f, "Acceptance Rate"),
        }
    }
}
