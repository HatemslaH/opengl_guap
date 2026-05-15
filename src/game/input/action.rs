/// Игровые действия — задаёшь сам под свою игру.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum GameAction {
    MoveRight,
    MoveLeft,
    MoveForward,
    MoveBackward,
    RotateLeft,
    RotateRight,
    Jump,
}
