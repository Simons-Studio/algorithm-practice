pub fn run() {
    println!("Solve the Hanoi");
    let tower_0 = vec![8, 7, 6, 5, 4, 3, 2, 1];
    let tower_1 = Vec::new();
    let tower_2 = Vec::new();

    let mut hanoi = Hanoi { tower_0, tower_1, tower_2 };

    solve_hanoi(&mut hanoi);

    println!("{:?} {:?} {:?}", hanoi.tower_0, hanoi.tower_1, hanoi.tower_2);
}

fn solve_hanoi(hanoi: &mut Hanoi) {
    let n = hanoi.tower_0.len();
    hanoi.move_top_blocks_to_target_tower(n as i32, 0, 2);
}

struct Hanoi {
    tower_0: Vec<i32>,
    tower_1: Vec<i32>,
    tower_2: Vec<i32>,
}
impl Hanoi {
    fn move_top_blocks_to_target_tower(&mut self, n: i32, source: i32, target: i32) {
        if source < 0 || source > 2 || target < 0 || target > 2 { return; }
        let spare = 3 - source - target;
        if n < 1 {
            println!("Too few blocks");
        } else if n == 1 {
            println!("Move 1 block  from {source} to {target}");
            self.move_block(source, target);
        } else {
            println!("Move {n} blocks from {source} to {target}");
            // 1. Move n-1 blocks to spare
            self.move_top_blocks_to_target_tower(n-1, source, spare);
            // 2. Move nth block to target
            self.move_block(source, target);
            // 3. Move spare block to target
            self.move_top_blocks_to_target_tower(n-1, spare, target);
        }
    }

    fn move_block(&mut self, source: i32, target: i32) {
        let safe_source = source % 3;
        let block_option = match safe_source {
            0 => self.tower_0.pop(),
            1 => self.tower_1.pop(),
            2 => self.tower_2.pop(),
            _ => None
        };
        let Some(block) = block_option else {
            return;
        };
        
        let safe_target = target % 3;
        match safe_target {
            0 => self.tower_0.push(block),
            1 => self.tower_1.push(block),
            _ => self.tower_2.push(block),
        }
    }
}
