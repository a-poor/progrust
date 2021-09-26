mod progrust {

pub struct ProgressBar {
    pos:   u32,
    total: u32,
    width: u32
}

impl ProgressBar {

    pub fn new(total: Option<u32>, width: Option<u32>) -> ProgressBar {
        ProgressBar {
            pos: 0,
            total: total.unwrap_or(100),
            width: width.unwrap_or(80)
        }
    }

    pub fn inc(self: &mut ProgressBar) {
        self.inc_by(1)
    }

    pub fn inc_by(self: &mut ProgressBar, n: u32) {
        let new = self.pos + n;
        self.pos = if new > self.total {
            self.total
        } else {
            new
        }
    }

    pub fn print(self: &ProgressBar) {
        print!("{}", self.format_bar());
    }

    pub fn done(self: &ProgressBar) {
        self.print();
        println!("");
    }

    fn format_bar(self: &ProgressBar) -> String {
        let bar_pct = self.pos as f32 / self.total as f32;
        let bar_width = self.width - 6;
        let n_filled = (bar_width as f32 * bar_pct) as u32;
        let n_unfilled = bar_width - n_filled;

        let filled = "█".repeat(n_filled as usize);
        let unfilled = " ".repeat(n_unfilled as usize);

        format!("\r[{}{}] {:5.1}%", filled, unfilled, bar_pct * 100.0)
    }

}

}

use std::{thread, time::Duration};

fn main() {
    println!("Hello, world!");

    let mut pb = progrust::ProgressBar::new(Some(1000), None);
    pb.print();

    for _ in 0..1000 {
        pb.print();
        pb.inc();
        thread::sleep(Duration::from_millis(50));
    }
    pb.done();
    
}
