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
        self.player.next(self.size, op, self.enemies, &self.scraps);

        for enemy in self.enemies.into_iter() {
            enemy.next(self.size, self.player, &self.scraps);
        }

        for n in 0..self.enemies.len() {
            let pos = self.enemies[n].pos();
            let mut flag = false;
            for m in (n + 1)..self.enemies.len() {
                flag |= pos == self.enemies[m].pos();
            }
            if flag { self.scraps.push(pos) }
        }

        for n in 0..self.scraps.len() {
            let pos = self.scraps[n];
            loop {
                let dups = self.enemies.into_iter().position(|e| pos == e.pos());
                match dups {
                    Some(v) => {
                        self.enemies.remove(v);
                    },
                    None => break
                }
            }
        }
    }

    pub fn size(&self) -> Vec2 { self.size }

    pub fn at(&self, pos: Vec2) -> CellStatus {
        if pos == self.player.pos() { return CellStatus::Player(); }
        for scrap in self.scraps.iter() {
            if pos == *scrap { return CellStatus::Scrap(); }
        }
        for enemy in self.enemies.iter() {
            if pos == enemy.pos() { return CellStatus::Enemy(enemy.id()); }
        }
        return CellStatus::Empty();
    }

    pub fn status(&self) -> GameStatus {
        if self.enemies.len() == 0 { return GameStatus::GameClear(); }
        for enemy in self.enemies.iter() {
            if self.player.pos() == enemy.pos() { return GameStatus::GameOver(); }
        }
        return GameStatus::InProgress();
    }
}