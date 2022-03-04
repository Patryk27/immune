pub use self::TutorialStage::*;

pub const TUTORIAL_STAGES: &[TutorialStage] = &[
    WeclomePage,
    LymphNodeIntroduction,
    LymhNodeConnecting,
    LyphNodeUnitsProduction,
    ResourcesIntroduction,
    UnitsControls,
    EnemiesIntroduction,
    CombatInstuctions,
    EnemiesUnfairAdventageExplanation,
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
    LymhNodeConnecting,
    LyphNodeUnitsProduction,
    ResourcesIntroduction,
    UnitsControls,
    EnemiesIntroduction,
    CombatInstuctions,
    EnemiesUnfairAdventageExplanation,
    Gameplay,
}

impl TutorialStage {
    pub fn description(&self) -> &str {
        match *self {
            WeclomePage => "Welcome!",
            LymphNodeIntroduction => "Lymph node introduction",
            LymhNodeConnecting => "Connecting Lymph nodes",
            LyphNodeUnitsProduction => "Producing unites via Lymph nodes",
            ResourcesIntroduction => "Resources",
            UnitsControls => "Cell units controls",
            EnemiesIntroduction => "Beware enemies",
            CombatInstuctions => "Kill them",
            EnemiesUnfairAdventageExplanation => "... or they will kill you",
            Gameplay => "",
        }
    }
}
