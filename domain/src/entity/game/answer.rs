// はい、いいえ、無関係,終了
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Answer {
    Yes,
    No,
    Irrelevant,
    Finish
}