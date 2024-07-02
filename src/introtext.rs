use macroquad::text::{measure_text, Font};
use super::FONT_SIZE;

#[derive(Default)]
pub struct IntroText {
    raw:      Box<str>,
    print_to: usize,
    timer:    u8,
    cache:    Vec<String>,
}

impl IntroText {
    const FRAMES_PER_CHAR: u8 = 4;

    pub fn init(&mut self, bucko_amount: u8, grid_size: u16) {
        self.raw = format!(
            "Creative Computing Buckopia, New Jrsey\n\
            The object of this game is to find the {} bucko{} hidden on a {} by {} \
            grid. \nYou get 10 tries. After each try, I will \nkill \nyou.",
            bucko_amount,
            if bucko_amount > 1 { "s" } else { "" },
            grid_size,
            grid_size,
        )
        .into();

        // IntroText {
        //     raw,
        //     print_to: 0,
        //     timer: 0,
        // }
    }

    pub fn get(&mut self, font: Option<&Font>, terminal_width: f32) -> &Vec<String> {
        if self.timer != Self::FRAMES_PER_CHAR {
            if self.print_to < self.raw.len() {
                self.timer += 1;
            }
            return &self.cache;
        }
        self.print_to += 1;
        self.timer = 0;

        // "it's not the best but it works"
        // the allocations...
        let mut output = Vec::<String>::new();
        for line in self.raw[..self.print_to].lines() {
            if measure_text(line, font, FONT_SIZE, 1.0).width < terminal_width {
                output.push(line.to_string());
            } else {
                let mut total_width = 0.0_f32;
                let mut new_line = String::new();
                for word in line.split_inclusive(' ') {
                    total_width += measure_text(word, font, FONT_SIZE, 1.0).width;
                    if total_width > terminal_width {
                        output.push(new_line);
                        new_line = word.to_string();
                        total_width = 0.0;
                    } else {
                        new_line += word;
                    }
                }
                output.push(new_line)
            }
        }
        self.cache = output;
        &self.cache
    }
}

#[cfg(test)]
mod intro_text_test {
    use macroquad::{
        color::WHITE,
        file::set_pc_assets_folder,
        text::{draw_text_ex, load_ttf_font, TextParams},
        window::next_frame,
    };

    use super::IntroText;

    #[macroquad::test("Test")]
    async fn intro_text_test() {
        set_pc_assets_folder("assets");
        let font = load_ttf_font("C64_Pro_Mono-STYLE.ttf").await.unwrap();
        let mut intro_text = IntroText::default();
        intro_text.init(4, 10);

        loop {
            for (i, line) in intro_text.get(Some(&font), 240.0).iter().enumerate() {
                draw_text_ex(
                    line,
                    0.0,
                    40.0 + 12.0 * i as f32,
                    TextParams {
                        font: Some(&font),
                        font_size: 12,
                        color: WHITE,
                        ..Default::default()
                    },
                );
            }
            next_frame().await;
        }
    }
}
