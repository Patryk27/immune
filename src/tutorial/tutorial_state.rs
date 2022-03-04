pub use self::TutorialStage::*;

pub const TUTORIAL_STAGES: &[TutorialStage] = &[
    WeclomePage,
    LymphNodeIntroduction,
    ResourcesIntroduction,
    EnemiesIntroduction,
    CombatInstuctions,
    EnemiesUnfairAdventage,
    Gameplay,
];

#[derive(Default)]
pub struct TutorialState {
    pub stage: usize,
}

#[derive(Copy, Clone, Debug)]
pub enum TutorialStage {
    WeclomePage,
    LymphNodeIntroduction,
    ResourcesIntroduction,
    EnemiesIntroduction,
    CombatInstuctions,
    EnemiesUnfairAdventage,
    Gameplay,
}

impl TutorialStage {
    pub fn description(&self) -> &str {
        match *self {
            WeclomePage => "Welcome!",
            LymphNodeIntroduction => "Lymph node introduction",
            ResourcesIntroduction => "Resources",
            EnemiesIntroduction => "Beware enemies",
            CombatInstuctions => "Kill them",
            EnemiesUnfairAdventage => "... before they kill you",
            Gameplay => "",
        }
    }
}
