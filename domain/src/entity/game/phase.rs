// ゲームフェース
// 1. 役割決め
// 2. 問題出題
// 3. 問答
// 4. 終了

#[derive(Debug,Clone, Eq, PartialEq)]
pub enum GamePhase{
    RoleDetermination,
    RaisingQuestion,
    Questioning,
    Result,
}