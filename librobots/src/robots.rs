use {PlayerTrait, EnemyTrait, Operations, GameStatus, CellStatus, Vec2};

pub struct Robots<'a> {
    size: Vec2,
    player: &'a mut Box<PlayerTrait>,
    enemies: &'a mut Vec<Box<EnemyTrait>>,
    scraps: Vec<Vec2>,
}

impl<'a> Robots<'a> {
    pub fn new(size: Vec2, player: &'a mut Box<PlayerTrait>, enemies: &'a mut Vec<Box<EnemyTrait>>) -> Robots<'a> {
        Robots { size, player, enemies, scraps: Vec::new() }
    }

    pub fn next(&mut self, op: Operations) {
        self.player.next(self.size, op, self.enemies);

        for enemy in self.enemies.into_iter() {
            enemy.next(self.size, self.player);
        }
    }

    pub fn size(&self) -> Vec2 { self.size }

    pub fn at(&self, pos: Vec2) -> CellStatus {
        if pos == self.player.pos() { return CellStatus::Player(); }
        for i in 0..self.enemies.len() {
            if pos == self.enemies[i].pos() { return CellStatus::Enemy(self.enemies[i].id()); }
        }
        return CellStatus::Empty();
    }

    pub fn status(&self) -> GameStatus {
        return GameStatus::InProgress();
    }
}