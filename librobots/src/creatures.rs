use {Operations, Vec2};

pub trait PlayerTrait {
    fn pos(&self) -> Vec2;
    fn next(&mut self, size: Vec2, op: Operations, enemies: &Vec<Box<EnemyTrait>>, scraps: &Vec<Vec2>);
}

pub trait EnemyTrait {
    fn id(&self) -> u64;
    fn pos(&self) -> Vec2;
    fn next(&mut self, size: Vec2, player: &Box<PlayerTrait>, scraps: &Vec<Vec2>);
}
