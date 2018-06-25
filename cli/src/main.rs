extern crate librobots;
extern crate rand;

use std::io::Write;
use librobots::{Robots, PlayerTrait, EnemyTrait, Operations, GameStatus, CellStatus, Vec2};
use rand::prelude::{random};

struct SimplePlayer {
    pos: Vec2
}

impl SimplePlayer {
    fn new(x: usize, y: usize) -> SimplePlayer {
        SimplePlayer { pos: Vec2::new(x, y) }
    }
}

impl PlayerTrait for SimplePlayer {
    fn pos(&self) -> Vec2 { self.pos }

    fn next(&mut self, size: Vec2, op: Operations, enemies: &Vec<Box<EnemyTrait>>, scraps: &Vec<Vec2>) -> Result<(), ()> {
        let upper_bound = self.pos.y == 0;
        let lower_bound = self.pos.y == size.y - 1;
        let left_bound = self.pos.x == 0;
        let right_bound = self.pos.x == size.x - 1;

        let mut pos = self.pos;
        let (ex, ey) = (Vec2::new(1, 0), Vec2::new(0, 1));

        match op {
            Operations::Up() => {
                if upper_bound { return Err(()); }
                pos = self.pos - ey;
            }
            Operations::Down() => {
                if lower_bound { return Err(()); }
                pos = self.pos + ey;
            }
            Operations::Left() => {
                if left_bound { return Err(()); }
                pos = self.pos - ex;
            }
            Operations::Right() => {
                if right_bound { return Err(()); }
                pos = self.pos + ex;
            }
            Operations::UpperLeft() => {
                if upper_bound || left_bound { return Err(()); }
                pos = self.pos - ex - ey;
            }
            Operations::UpperRight() => {
                if upper_bound || right_bound { return Err(()); }
                pos = self.pos + ex - ey;
            }
            Operations::LowerLeft() => {
                if lower_bound || left_bound { return Err(()); }
                pos = self.pos - ex + ey;
            }
            Operations::LowerRight() => {
                if lower_bound || right_bound { return Err(()); }
                pos = self.pos + ex + ey;
            }
            Operations::Warp() => {
                loop {
                    let x = random::<usize>() % size.x;
                    let y = random::<usize>() % size.y;
                    pos = Vec2::new(x, y);
                    for enemy in enemies.iter() {
                        if enemy.pos() == pos { continue; }
                    }
                    break;
                }
            }
            _ => ()
        }

        for enemy in enemies.iter() {
            if pos == enemy.pos() { return Err(()); }
        }

        for scrap in scraps.iter() {
            if pos == *scrap { return Err(()); }
        }

        self.pos = pos;
        Ok(())
    }
}

struct SimpleEnemy {
    pos: Vec2
}

impl SimpleEnemy {
    fn new(x: usize, y: usize) -> SimpleEnemy {
        SimpleEnemy { pos: Vec2::new(x, y) }
    }
}

impl EnemyTrait for SimpleEnemy {
    fn id(&self) -> u64 { 1 }
    fn pos(&self) -> Vec2 { self.pos }

    fn next(&mut self, _: Vec2, player: &Box<PlayerTrait>, _: &Vec<Vec2>) -> Result<(), ()> {
        if player.pos().x < self.pos.x { self.pos -= Vec2::ex(); }
        if player.pos().x > self.pos.x { self.pos += Vec2::ex(); }
        if player.pos().y < self.pos.y { self.pos -= Vec2::ey(); }
        if player.pos().y > self.pos.y { self.pos += Vec2::ey(); }

        return Ok(());
    }
}

fn display(g: &Robots) {
    let (width, height) = (g.size().x, g.size().y);

    println!("+{}+", "-".repeat(width));
    for y in 0..height {
        print!("|");
        for x in 0..width {
            match g.at(Vec2::new(x, y)) {
                CellStatus::Player() => { print!("@"); }
                CellStatus::Enemy(_) => { print!("+"); }
                CellStatus::Empty() => { print!(" "); }
                CellStatus::Scrap() => { print!("*")}
            }
        }
        println!("|");
    }
    println!("+{}+", "-".repeat(width));
}

fn get_operation(prompt: &str) -> Operations {
    loop {
        let mut s = String::new();

        print!("{}", prompt);
        std::io::stdout().flush().expect("");
        std::io::stdin().read_line(&mut s).expect("");

        match s.trim_right_matches("\n") {
            "u" | "2" => return Operations::Up(),
            "m" | "8" => return Operations::Down(),
            "h" | "4" => return Operations::Left(),
            "k" | "6" => return Operations::Right(),
            "y" | "1" => return Operations::UpperLeft(),
            "i" | "3" => return Operations::UpperRight(),
            "n" | "7" => return Operations::LowerLeft(),
            "," | "9" => return Operations::LowerRight(),
            " " | "o" | "5" => return Operations::Wait(),
            "j" | "0" => return Operations::Warp(),
            _ => ()
        }
    }
}

fn create_entities(size: Vec2, num: usize) -> (Box<PlayerTrait>, Vec<Box<EnemyTrait>>) {
    let px = size.x / 2;
    let py = size.y / 2;

    let player = Box::new(SimplePlayer::new(px, py));

    let mut enemies = Vec::new() as Vec<Box<EnemyTrait>>;

    for n in 0..num {        
        while enemies.len() == n {
            let x = random::<usize>() % size.x;
            let y = random::<usize>() % size.y;

            if x == player.pos().x && y == player.pos().y { continue; }
            for enemy in enemies.iter() { if x == enemy.pos().x && y == enemy.pos().y { continue; } }

            enemies.push(Box::new(SimpleEnemy::new(x, y))); 
        }
    }

    return (player, enemies);
}


fn main() {
    let size = Vec2::new(75, 25);
    let mut level = 1;
    let mut point = 0;

    loop {
        let num = std::cmp::min(5 * level, 40);
        let (player, enemies) = create_entities(size, num);
        let mut game = Robots::new(size, player, enemies);

        while game.status() == GameStatus::InProgress() {
            display(&game);
            let prompt = format!("(level:{} score:{}) : ", level, point + game.point());
            let op = get_operation(&prompt);
            game.next(op);
        }

        match game.status() {
            GameStatus::GameClear() => {
                point += game.point() + level * 10;
                level += 1;
            },
            GameStatus::GameOver() => {
                point += game.point();
                break;
            },
            _ => panic!("unreachable!"),
        }
    }
}
