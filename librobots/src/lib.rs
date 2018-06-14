mod constants;
mod vec2;
mod robots;
mod creatures;

pub use constants::{Operations, GameStatus, CellStatus};
pub use vec2::Vec2;
pub use robots::Robots;
pub use creatures::{PlayerTrait, EnemyTrait};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
