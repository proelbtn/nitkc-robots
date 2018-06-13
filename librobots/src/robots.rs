use {PlayerTrait, EnemyTrait, Operations, Vec2};

pub struct Robots<'a> {
    size: Vec2,
    player: &'a mut Box<PlayerTrait>,
    enemies: &'a mut Vec<Box<EnemyTrait>>,
}

impl<'a> Robots<'a> {
    pub fn new(size: Vec2, player: &'a mut Box<PlayerTrait>, enemies: &'a mut Vec<Box<EnemyTrait>>) -> Robots<'a> {
        Robots { size, player, enemies }
    }

    pub fn next(&mut self, op: Operations) {
        self.player.next(self.size, op, self.enemies);

        for enemy in self.enemies.into_iter() {
            enemy.next(self.size, self.player);
        }
    }
}