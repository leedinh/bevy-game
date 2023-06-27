use bevy::prelude::*;

#[derive(Resource)]
pub struct  Score {
    pub value: u32,
}

impl Default for Score {
    fn  default() -> Score {
            Score { value: 0 }
}
}

#[derive(Resource, Debug)]
pub struct HighScore {
    pub scores: Vec<(String, u32)>

}

impl Default for HighScore {
    fn default() -> HighScore {
        HighScore { 
            scores: Vec::new(),
        }
    }
}
