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

pub enum GameStates {
    GameOver(),
    GameClear(),
    InProgress(),
}

pub enum CellStates {
    Player(),
    Enemy(u64),
    Scrap(),
    Empty(),
}