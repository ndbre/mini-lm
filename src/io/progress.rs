use std::io::{self, Write};

// This is code copied from one of my previous projects, so that is why it might seem a little different
// compared to everything else.

pub struct ProgressBarStyle {
    // Character used to fill the completed portion
    pub fill: char,
    // Character used for the remaining portion
    pub empty: char,
    // Left bracket character
    pub left: char,
    // Right bracket character
    pub right: char,
    // Total width of the bar
    pub width: usize,
    // Whether to show the percentage
    pub show_percent: bool,
    // Optional prefix label shown before the bar
    pub label: Option<String>,
}

impl Default for ProgressBarStyle {
    fn default() -> Self {
        Self {
            fill: '#',
            empty: ' ',
            left: '[',
            right: ']',
            width: 40,
            show_percent: true,
            label: None,
        }
    }
}

pub struct ProgressBar {
    total: u64,
    style: ProgressBarStyle,
}

impl ProgressBar {
    pub fn new(total: u64, style: ProgressBarStyle) -> Self {
        Self { total, style }
    }

    pub fn update(&self, current: u64) {
        let current = current.min(self.total);
        let ratio = if self.total == 0 {
            1.0
        } else {
            current as f64 / self.total as f64
        };

        let filled = (ratio * self.style.width as f64).round() as usize;
        let empty = self.style.width.saturating_sub(filled);

        let bar: String = std::iter::repeat(self.style.fill)
            .take(filled)
            .chain(std::iter::repeat(self.style.empty).take(empty))
            .collect();

        let mut line = String::new();

        if let Some(ref label) = self.style.label {
            line.push_str(label);
            line.push(' ');
        }

        line.push(self.style.left);
        line.push_str(&bar);
        line.push(self.style.right);

        if self.style.show_percent {
            line.push_str(&format!(" {:>6.2}%", ratio * 100.0));
        }

        line.push_str(&format!(" ({}/{})", current, self.total));

        print!("\r{}", line);
        io::stdout().flush().unwrap();
    }

    pub fn finish(&self, message: String) {
        self.update(self.total);
        println!("\n{}", message);
    }
}
