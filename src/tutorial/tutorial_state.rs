pub use self::TutorialStage::*;

pub const TUTORIAL_STAGES: &[TutorialStage] = &[
    LymphNodeIntroduction,
    LymhNodeConnecting,
    LyphNodeUnitsProduction,
    ResourcesIntroduction,
    UnitsControls,
    EnemiesIntroduction,
    EnemyWavesIntroduction,
    CombatInstuctions,
    EnemiesUnfairAdventageExplanation,
    Gameplay,
];

#[derive(Default)]
pub struct TutorialState {
    _stage: usize,
}

pub enum TutorialStage {
    LymphNodeIntroduction,
    LymhNodeConnecting,
    LyphNodeUnitsProduction,
    ResourcesIntroduction,
    UnitsControls,
    EnemiesIntroduction,
    EnemyWavesIntroduction,
    CombatInstuctions,
    EnemiesUnfairAdventageExplanation,
    Gameplay,
}
