use super::player::Player;

#[derive(Debug, Clone)]
/// メンバーの役割
pub enum Member {
    Questioner(Player),
    Guesser(Player),
}
