pub enum Operations {
    Up(),
    Down(),
    Left(),
    Right(),
    UpperLeft(),
    UpperRight(),
    LowerLeft(),
    LowerRight(),
    Wait(),
    Warp(),
}

#[derive(PartialEq)]
pub enum GameStatus {
    GameOver(),
    GameClear(),
    InProgress(),
}

#[derive(PartialEq)]
pub enum CellStatus {
    Player(),
    Enemy(u64),
    Scrap(),
    Empty(),
}