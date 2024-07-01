use macroquad::prelude::*;

// use super::{BAR_SIZE, FONT_SIZE, MARGIN};

// pub struct _IntroText<'a> {
//     text:  Vec<String>,
//     chars: std::str::Chars<'a>,
//     raw:   String,
//     timer: u8,
//     font:  Option<&'a Font>,
// }

// impl<'a> _IntroText<'a> {
//     const FRAMES_PER_CHAR: u8 = 4;

//     pub fn empty() -> _IntroText<'a> {
//         _IntroText {
//             text:  Vec::<String>::new(),
//             chars: "".chars(),
//             raw:   String::new(),
//             timer: 0,
//             font:  None,
//         }
//     }

//     pub fn fill(&'a mut self, font: Option<&'a Font>, bucko_amount: u8, grid_size: u16) {
//         self.raw = format!(
//             "Creative Computing Buckopia, New Jrsey\n\
//             The object of this game is to find the {} bucko{} hidden on a {} by {} \
//             grid. \nYou get 10 tries. After each try, I will \nkill \nyou.",
//             bucko_amount,
//             if bucko_amount > 1 { "s" } else { "" },
//             grid_size,
//             grid_size,
//         );
//         self.chars = self.raw.chars();
//         self.font = font;
//     }

//     pub fn update(&'a mut self) {
//         self.timer += 1;
//         if self.timer == _IntroText::FRAMES_PER_CHAR {
//             self.timer = 0;
//             return;
//         }

//         if let Some(char) = self.chars.next() {
//             let terminal_width = screen_width() - 4.0 * MARGIN;
//             let word = self
//                 .chars
//                 .clone()
//                 .take_while(|c| c.is_alphanumeric())
//                 .collect::<String>();
//             let last_line = self.text.last_mut().unwrap();
//             let word_dims = measure_text(&word, self.font, FONT_SIZE, 1.0);
//             let line_dims = measure_text(last_line, self.font, FONT_SIZE, 1.0);
//             if line_dims.width + word_dims.width < terminal_width && char != '\n' {
//                 last_line.push(char);
//             } else if char.is_whitespace() {
//                 self.text.push(String::new())
//             } else {
//                 self.text.push(char.to_string());
//             }
//         }
//     }

//     pub fn draw(&'a self, text_params: TextParams) {
//         let text_height = measure_text("dp", self.font, FONT_SIZE, 1.0).height;
//         for (i, line) in self.text.iter().enumerate() {
//             let spacing = (i + 1) as f32 * (text_height + MARGIN) + BAR_SIZE + screen_width();
//             draw_text_ex(line, 2.0 * MARGIN, spacing, text_params.clone())
//         }
//     }
// }

#[derive(Default)]
pub struct IntroText {
    raw:      Box<str>,
    print_to: usize,
    timer:    u8,
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

    pub fn get(&mut self) -> &str {
        if self.print_to <= self.raw.len() {
            self.timer += 1;
        }
        if self.timer == Self::FRAMES_PER_CHAR {
            self.print_to += 1;
            self.timer = 0;
        }
        &self.raw[..self.print_to]
    }

    pub fn print(terminal_width: f32) -> Vec<String> {
        todo!()
    }
}
