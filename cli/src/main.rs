extern crate librobots;
extern crate rand;

use std::io::Write;
use librobots::{Robots, PlayerTrait, EnemyTrait, Operations, GameStatus, CellStatus, Vec2};
use rand::prelude::{random};

struct SimplePlayer {
    pos: Vec2
}

impl SimplePlayer {
    fn new(x: u64, y: u64) -> SimplePlayer {
        SimplePlayer { pos: Vec2::new(x, y) }
    }
}

impl PlayerTrait for SimplePlayer {
    fn pos(&self) -> Vec2 { self.pos }

    fn next(&mut self, size: Vec2, op: Operations, enemies: &Vec<Box<EnemyTrait>>, scraps: &Vec<Vec2>) {
        let upper_bound = self.pos.y == 0;
        let lower_bound = self.pos.y == size.y;
        let left_bound = self.pos.x == 0;
        let right_bound = self.pos.x == size.x;

        let mut pos = self.pos;
        let (ex, ey) = (Vec2::new(1, 0), Vec2::new(0, 1));

        match op {
            Operations::Up() => {
                if upper_bound { return; }
                pos = self.pos - ey;
            }
            Operations::Down() => {
                if upper_bound { return; }
                pos = self.pos + ey;
            }
            Operations::Left() => {
                if upper_bound { return; }
                pos = self.pos - ex;
            }
            Operations::Right() => {
                if upper_bound { return; }
                pos = self.pos + ex;
            }
            Operations::UpperLeft() => {
                if upper_bound { return; }
                pos = self.pos - ex - ey;
            }
            Operations::UpperRight() => {
                if upper_bound { return; }
                pos = self.pos + ex - ey;
            }
            Operations::LowerLeft() => {
                if upper_bound { return; }
                pos = self.pos - ex + ey;
            }
            Operations::LowerRight() => {
                if upper_bound { return; }
                pos = self.pos + ex + ey;
            }
            Operations::Warp() => {
                loop {
                    let x = random::<u64>() % size.x;
                    let y = random::<u64>() % size.y;
                    pos = Vec2::new(x, y);
                    for enemy in enemies.iter() {
                        if enemy.pos() == pos { continue; }
                    }
                    break;
                }
            }
            _ => ()
        }

        self.pos = pos;
    }
}

struct SimpleEnemy {
    pos: Vec2
}

impl SimpleEnemy {
    fn new(x: u64, y: u64) -> SimpleEnemy {
        SimpleEnemy { pos: Vec2::new(x, y) }
    }
}

impl EnemyTrait for SimpleEnemy {
    fn id(&self) -> u64 { 1 }
    fn pos(&self) -> Vec2 { self.pos }

    fn next(&mut self, size: Vec2, player: &Box<PlayerTrait>, scraps: &Vec<Vec2>) {
        let (px, py) = (player.pos().x, player.pos().y);
        let mut pos = self.pos;
        let (ex, ey) = (Vec2::new(1, 0), Vec2::new(0, 1));

        if px < self.pos.x { pos = pos - ex; }
        if px > self.pos.x { pos = pos + ex; }
        if py < self.pos.y { pos = pos - ey; }
        if py > self.pos.y { pos = pos + ey; }

        self.pos = pos;
    }
}

fn display(g: &Robots) {
    let (width, height) = (g.size().x, g.size().y);

    println!("+{}+", "-".repeat(width as usize));
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
    println!("+{}+", "-".repeat(width as usize));
}

fn get_operation(prompt: &str) -> Operations {
    loop {
        let mut s = String::new();

        print!("{}", prompt);
        std::io::stdout().flush();
        std::io::stdin().read_line(&mut s);

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

fn new_enemies(p: Vec2, s: Vec2, n: usize) -> Vec<Box<EnemyTrait>> {
    let mut v: Vec<Box<EnemyTrait>> = Vec::new();
    for _ in 0..n {        
        loop {
            let x = random::<u64>() % s.x;
            let y = random::<u64>() % s.y;
            let pos = Vec2::new(x, y);
            if p == pos { continue; }
            for enemy in v.iter() {
                if enemy.pos() == pos { continue; }
            }
            v.push(Box::new(SimpleEnemy::new(x, y))); 
            break;
        }
    }
    return v;
}


fn main() {
    let size = Vec2::new(45, 15);
    let mut level = 1;

    loop {
        let mut p: Box<PlayerTrait> = Box::new(SimplePlayer::new(size.x / 2, size.y / 2));
        let mut es: Vec<Box<EnemyTrait>> = new_enemies(p.pos(), size, std::cmp::min(5 * level, 40));
        let mut g = Robots::new(size, &mut p, &mut es);

        while g.status() == GameStatus::InProgress() {
            display(&g);
            let op = get_operation("$ ");
            g.next(op);
        }

        match g.status() {
            GameStatus::GameClear() => {
            },
            GameStatus::GameOver() => {
            },
            _ => panic!("unreachable sequence"),
        }
    }
}
