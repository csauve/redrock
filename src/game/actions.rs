
#[non_exhaustive]
pub enum PlayerAction {
    Left(bool),
    Right(bool),
    Forward(bool),
    Back(bool),
    Jump(bool),
    Boost(bool),
    Crouch(bool),
    AimDelta(f32, f32),
    Quit,
}