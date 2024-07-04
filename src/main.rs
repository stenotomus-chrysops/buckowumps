use introtext::IntroText;
use macroquad::{
    audio::{load_sound, play_sound, set_sound_volume, PlaySoundParams, Sound},
    experimental::animation::{AnimatedSprite, Animation},
    prelude::*,
    ui::{hash, root_ui, widgets, Skin},
};
use std::collections::VecDeque;
use titlebuckos::TitleBuckos;
// use miniquad::window::schedule_update;

mod introtext;
mod titlebuckos;

fn config() -> Conf {
    miniquad::conf::Conf {
        window_title: "BuckoWump".to_owned(),
        window_width: 360,
        window_height: 640,
        fullscreen: false,
        sample_count: 0,
        window_resizable: true,
        platform: miniquad::conf::Platform {
            linux_backend: miniquad::conf::LinuxBackend::WaylandOnly,
            framebuffer_alpha: false,
            swap_interval: None,
            // blocking_event_loop: true,
            ..Default::default()
        },
        // icon: Some(render::icon::set()),
        ..Default::default()
    }
    // macroquad::conf::Conf {
    //     miniquad_conf: miniconf,
    //     update_on:     Some(macroquad::conf::UpdateTrigger {
    //         mouse_motion: true,
    //         mouse_down: true,
    //         specific_key: Some(vec![KeyCode::Enter, KeyCode::KpEnter]),
    //         ..Default::default()
    //     }),
    // }
}

enum GameState {
    MainMenu,
    Playing,
    GameOver,
}

struct Ami {
    cute: bool,

    position: Vec2,
    rotation: f32,
    texture:  Texture2D,
    sprite:   AnimatedSprite,

    follow_path: VecDeque<(Vec2, f32)>,

    // TODO idk come up with a better way
    got_bucko: bool,
    face_left: bool,
    visible:   bool,
    // state_machine: StateMachine<Ami>,
}

// impl scene::Node for Ami {
//      fn update(mut _node: RefMut<Self>) {}
//
//     fn draw(mut node: RefMut<Self>) {
//         node.sprite.update();
//
//         draw_texture_ex(
//             &node.texture,
//             node.position.x - 64.0,
//             node.position.y - 64.0,
//             WHITE,
//             DrawTextureParams {
//                 dest_size: Some(Vec2::splat(80.0)),
//                 source: Some(node.sprite.frame().source_rect),
//                 rotation: node.rotation,
//                 flip_x: false,
//                 ..Default::default()
//             },
//         )
//     }
// }

impl Ami {
    const JUMP_STEPS: u16 = 30;
    // const JUMP_TIME: f32 = 2.0;

    const REVEAL: usize = 0;
    const WAIT: usize = 1;
    const JUMP: usize = 2;
    const LAND: usize = 3;
    const CATCH: usize = 4;
    const LOOK: usize = 5;
    const CLIMB: usize = 6;
    const GAMEOVER: usize = 7;

    async fn init() -> Result<Ami, macroquad::Error> {
        let texture = load_texture("ami.png").await?;
        let sprite = AnimatedSprite::new(
            128,
            128,
            &[
                Animation {
                    name:   "reveal".to_string(),
                    row:    0,
                    frames: 4,
                    fps:    20,
                },
                Animation {
                    name:   "wait".to_string(),
                    row:    1,
                    frames: 4,
                    fps:    4,
                },
                Animation {
                    name:   "jump".to_string(),
                    row:    2,
                    frames: 4,
                    fps:    6,
                },
                Animation {
                    name:   "land".to_string(),
                    row:    3,
                    frames: 4,
                    fps:    6,
                },
                Animation {
                    name:   "catch".to_string(),
                    row:    4,
                    frames: 4,
                    fps:    6,
                },
                Animation {
                    name:   "look".to_string(),
                    row:    5,
                    frames: 4,
                    fps:    6,
                },
                Animation {
                    name:   "climb".to_string(),
                    row:    6,
                    frames: 4,
                    fps:    6,
                },
                Animation {
                    name:   "gameover".to_string(),
                    row:    7,
                    frames: 4,
                    fps:    6,
                },
            ],
            false,
        );

        // let mut state_machine = StateMachine::new();
        // state_machine.add_state(Self::IDLE, State::new().update(Self::update));
        // state_machine.add_state(Self::PEEK, State::new().coroutine(Self::jump));
        // state_machine.add_state(Self::JUMP, State::new().coroutine(Self::jump));
        // state_machine.add_state(Self::CLIMB, State::new().coroutine(Self::climb));

        Ok(Ami {
            cute: true,
            visible: false,
            position: vec2(screen_width() / 2.0, BAR_SIZE),
            rotation: 0.0,
            follow_path: VecDeque::with_capacity(Ami::JUMP_STEPS.into()),
            texture,
            sprite,
            got_bucko: false,
            face_left: false,
            // state_machine,
        })
    }

    fn draw(&self) {
        draw_texture_ex(
            &self.texture,
            self.position.x - 40.0,
            self.position.y - 40.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::splat(80.0)),
                source: Some(self.sprite.frame().source_rect),
                rotation: self.rotation,
                flip_x: self.face_left,
                ..Default::default()
            },
        )
    }

    fn jump(&mut self, target: Vec2) {
        self.face_left = target.x < 0.0;
        self.follow_path = (1..=Ami::JUMP_STEPS)
            .map(|s| {
                let x = s as f32 / Ami::JUMP_STEPS as f32;
                let y = 2.0 * x.powi(2) - x;
                let dy = 4.0 * x - 1.0;
                // This is a goddamn mess
                (
                    vec2(x, y) * target + vec2(screen_width() / 2.0, 40.0),
                    (dy.tanh() - std::f32::consts::FRAC_PI_2) * target.x.signum(),
                )
            })
            .collect();
        // println!("{:?}", self.follow_path.iter().last().unwrap());
        self.sprite.set_animation(Ami::JUMP);
        self.sprite.set_frame(0);
    }

    fn update(&mut self, sfx_climb: &Sound, sfx_volume: f32) {
        // Am I using animations as a state machine? Is this bad!?
        // Passing sfx ref, answer absolutely yes wtf am I writing
        // omg it needs the volume parameters too...
        match self.sprite.current_animation() {
            Ami::REVEAL if self.sprite.is_last_frame() => {
                self.sprite.set_animation(Ami::WAIT);
                self.sprite.set_frame(0);
            }
            Ami::JUMP => {
                // One new position per frame, for now (forever)
                // let ft = get_frame_time();
                // let past_frames =
                //     (get_frame_time() / (Ami::JUMP_TIME / Ami::JUMP_STEPS as f32)).floor() as usize;
                if let Some((dest, rot)) = self.follow_path.pop_front() {
                    self.position = dest;
                    self.rotation = rot;
                } else {
                    self.rotation = 0.0;
                    self.sprite.set_animation(Ami::LAND);
                    self.sprite.set_frame(0);
                }
            }
            Ami::LAND if self.sprite.is_last_frame() => {
                if self.got_bucko {
                    self.got_bucko = false;
                    self.sprite.set_animation(Ami::CATCH);
                    self.sprite.set_frame(0);
                    self.got_bucko = false;
                } else {
                    self.sprite.set_animation(Ami::LOOK);
                    self.sprite.set_frame(0);
                }
                self.face_left = false;
            }
            Ami::CATCH if self.sprite.is_last_frame() => {
                self.sprite.set_animation(Ami::LOOK);
                self.sprite.set_frame(0);
            }
            Ami::LOOK if self.sprite.is_last_frame() => {
                self.sprite.set_animation(Ami::CLIMB);
                self.sprite.set_frame(0);
                play_sound(
                    sfx_climb,
                    PlaySoundParams {
                        looped: false,
                        volume: sfx_volume / 100.0,
                    },
                );
            }
            Ami::CLIMB => {
                let ami_pos = (self.position, 0.0);
                let silk_top = self.follow_path.iter().last().unwrap_or(&ami_pos).0;
                draw_line(
                    self.position.x,
                    self.position.y,
                    silk_top.x,
                    silk_top.y,
                    1.0,
                    WHITE,
                );
                if silk_top.y > -128.0 {
                    let next_silk = silk_top - vec2(0.0, 32.0);
                    self.follow_path.push_back((next_silk, 0.0));
                } else if let Some((dest, _)) = self.follow_path.pop_front() {
                    self.position = dest;
                } else {
                    self.position = vec2(screen_width() / 2.0, BAR_SIZE);
                    self.sprite.set_animation(Ami::REVEAL);
                    self.sprite.set_frame(0);
                }
            }
            _ => self.sprite.update(),
        }
    }

    // fn update(_node: &mut RefMut<Ami>, _dt: f32) {
    //     // let handle = node.handle();
    // }

    // pub fn reveal(node: &mut RefMut<Ami>) -> Coroutine {
    //     let handle = node.handle();
    //     let coroutine = async move {
    //         {
    //             let mut node = scene::get_node(handle);
    //             node.sprite.set_animation(0);
    //             node.sprite.playing = true;
    //         }
    //         wait_seconds(1.0).await;
    //         {
    //             let mut node = scene::get_node(handle);
    //             node.sprite.set_animation(1);
    //         }
    //     };
    //     start_coroutine(coroutine)
    // }

    // fn jump(node: &mut RefMut<Ami>) -> Coroutine {
    //     let handle = node.handle();
    //     let coroutine = async move {};
    //     start_coroutine(coroutine)
    // }

    // fn climb(node: &mut RefMut<Ami>) -> Coroutine {
    //     let handle = node.handle();
    //     let coroutine = async move {};
    //     start_coroutine(coroutine)
    // }
}

#[derive(Default)]
struct Buckos {
    positions: Vec<U16Vec2>,
    captured:  Vec<bool>,
}

impl Buckos {
    fn new(amount: u8, grid_size: u16) -> Buckos {
        rand::srand((miniquad::date::now() * 10e7) as u64);
        let mut positions = Vec::<U16Vec2>::with_capacity(amount.into());
        for _new_bucko in 0..amount {
            let mut position: Option<U16Vec2> = None;
            // Prevent more than one bucko in a cell. AKA The Anti Bucko-Love Loop
            while position.map_or(true, |pos| positions.contains(&pos)) {
                // The linear algebra library has no u8 vectors
                // and macroquad rng has no u16 range generator
                // so you get this cast mess. I'm just happy to
                // have this apology comment be equally spaced.
                position = Some(u16vec2(
                    rand::gen_range::<u8>(0, grid_size as u8) as u16,
                    rand::gen_range::<u8>(0, grid_size as u8) as u16,
                ));
                // Check: uncomment to loop forever
                // position = Some(u16vec2(0, 0));
            }
            positions.push(position.unwrap())
        }

        Buckos {
            positions,
            captured: vec![false; amount as usize],
        }
    }

    fn distances(&mut self, cell: U16Vec2) -> Vec<Option<String>> {
        self.positions
            .iter()
            .enumerate()
            .map(|(i, pos)| {
                if self.captured[i] {
                    None
                } else if cell == *pos {
                    self.captured[i] = true;
                    Some("GOT".to_string())
                } else {
                    let distance = cell.as_vec2().distance(pos.as_vec2());
                    Some(format!("{:.1}", distance))
                }
            })
            .collect()
    }
}

struct Turn {
    target_cell: U16Vec2,
    distances:   Vec<Option<String>>,
}

const VERSION: &str = env!("CARGO_PKG_VERSION");

const BACK_COLOR: Color = color_u8!(45, 44, 154, 255);
const FORE_COLOR: Color = color_u8!(112, 110, 228, 255);
const BAR_SIZE: f32 = 60.0;
const MARGIN: f32 = 6.0;
const FONT_SIZE: u16 = 8;
const TURNS: u8 = 10;

#[macroquad::main(config)]
async fn main() -> Result<(), macroquad::Error> {
    set_pc_assets_folder("assets");
    let font = load_ttf_font("C64_Pro_Mono-STYLE.ttf").await?;
    let text_params = TextParams {
        font: Some(&font),
        font_size: FONT_SIZE,
        color: FORE_COLOR,
        ..Default::default()
    };

    let grid_skin = {
        let button_style = root_ui()
            .style_builder()
            .color(color_u8!(0, 0, 0, 0))
            .color_hovered(color_u8!(0, 0, 0, 64))
            .color_clicked(color_u8!(0, 0, 0, 64))
            .color_selected(color_u8!(255, 255, 255, 64))
            .color_selected_hovered(color_u8!(255, 255, 255, 64))
            .font(include_bytes!("../assets/C64_Pro_Mono-STYLE.ttf"))?
            .font_size(FONT_SIZE)
            .text_color(color_u8!(0, 0, 0, 0))
            .text_color_hovered(WHITE)
            .text_color_clicked(color_u8!(255, 255, 255, 64))
            .build();
        Skin {
            button_style,
            ..root_ui().default_skin()
        }
    };
    let ui_skin = {
        let button_style = root_ui()
            .style_builder()
            .color(color_u8!(0, 0, 0, 0))
            .color_clicked(color_u8!(0, 0, 0, 0))
            .color_hovered(color_u8!(0, 0, 0, 0))
            .color_inactive(color_u8!(0, 0, 0, 0))
            .color_selected(color_u8!(0, 0, 0, 0))
            .color_selected_hovered(color_u8!(0, 0, 0, 0))
            .font(include_bytes!("../assets/C64_Pro_Mono-STYLE.ttf"))?
            .font_size(FONT_SIZE)
            .text_color(FORE_COLOR)
            .build();
        Skin {
            button_style,
            ..root_ui().default_skin()
        }
    };
    let settings_skin = {
        let label_style = root_ui()
            .style_builder()
            .font(include_bytes!("../assets/C64_Pro_Mono-STYLE.ttf"))?
            .text_color(FORE_COLOR)
            .font_size(12)
            .build();
        let button_style = root_ui()
            .style_builder()
            .background(Image::empty())
            .font(include_bytes!("../assets/C64_Pro_Mono-STYLE.ttf"))?
            .text_color(FORE_COLOR)
            .font_size(16)
            .build();
        let window_style = root_ui()
            .style_builder()
            .color(BACK_COLOR)
            .font(include_bytes!("../assets/C64_Pro_Mono-STYLE.ttf"))?
            .text_color(FORE_COLOR)
            .font_size(12)
            .build();
        Skin {
            label_style,
            button_style,
            window_style,
            margin: MARGIN,
            ..root_ui().default_skin()
        }
    };

    let mut sfx_volume: f32 = 100.0;
    let mut bgm_volume: f32 = 100.0;
    let bgm = load_sound("chipper_chaps.wav").await?;
    let sfx_light = load_sound("light_switch.wav").await?;
    let sfx_jump = load_sound("jump.wav").await?;
    let sfx_climb = load_sound("climb.wav").await?;
    let sfx_gameover = load_sound("gameover.wav").await?;
    let sfx_win = load_sound("win.wav").await?;

    play_sound(
        &bgm,
        PlaySoundParams {
            looped: true,
            volume: 1.0,
        },
    );

    let mut ami = Ami::init().await?;
    let win_texture = load_texture("win.png").await?;

    let bucko_texture = load_texture("bucko.png").await?;
    let mut bucko_sprite = AnimatedSprite::new(
        64,
        64,
        &[Animation {
            name:   "exist".to_string(),
            row:    0,
            frames: 4,
            fps:    6,
        }],
        true,
    );

    let settings_texture = load_texture("settings.png").await?;
    let mut settings_on = false;

    let grid_size: u16 = 10;
    let mut bucko_amount: u8 = 4;

    let mut state = GameState::MainMenu;
    let mut selected_cell: Option<U16Vec2> = None;
    let mut buckos = Buckos::default();
    let mut log = Vec::<Turn>::with_capacity(TURNS as usize);

    let mut titlebuckos = TitleBuckos::default();
    titlebuckos.init(bucko_amount, screen_width() - 2.0 * MARGIN);

    let mut introtext = IntroText::default();

    loop {
        assert!(ami.cute);
        clear_background(FORE_COLOR);

        // Bar
        draw_rectangle(
            MARGIN,
            MARGIN,
            screen_width() - 2.0 * MARGIN,
            BAR_SIZE - MARGIN,
            BACK_COLOR,
        );

        draw_text_ex(
            "BuckoWumps",
            2.0 * MARGIN,
            BAR_SIZE / 2.0,
            TextParams {
                font: Some(&font),
                font_size: 12,
                color: FORE_COLOR,
                ..Default::default()
            },
        );

        draw_text_ex(
            VERSION,
            2.0 * MARGIN,
            BAR_SIZE / 2.0 + 16.0,
            TextParams {
                font: Some(&font),
                font_size: 8,
                color: FORE_COLOR,
                ..Default::default()
            },
        );

        // "Terminal" screen
        draw_rectangle(
            MARGIN,
            screen_width() + BAR_SIZE,
            screen_width() - 2.0 * MARGIN,
            screen_height() - (screen_width() + BAR_SIZE + MARGIN),
            BACK_COLOR,
        );

        let cell_size = (screen_width() - 2.0 * MARGIN) / grid_size as f32;
        let terminal_position = BAR_SIZE + screen_width();

        let enter_button = {
            let button_size = vec2(60.0, 18.0);
            let screen_size = vec2(screen_width(), screen_height());
            widgets::Button::new("EnterÙ")
                .position(screen_size - button_size - Vec2::splat(MARGIN))
                .size(button_size)
        };

        // Entry text
        {
            let mut entry_text = match state {
                GameState::MainMenu => "Click/Press Enter to begin!".to_string(),
                GameState::Playing => format!("Turn {}: ", log.len()),
                GameState::GameOver => "That was pog! Let's play again...".to_string(),
            };

            if let Some(cell) = selected_cell {
                entry_text.push_str(&format!("{},{}", cell.x, cell.y));
            } else if let GameState::Playing = state {
                entry_text.push_str("Select a cell");
            }

            draw_text_ex(
                &entry_text,
                2.0 * MARGIN,
                screen_height() - 2.0 * MARGIN,
                text_params.clone(),
            );
        }

        for (y, turn) in log.iter().enumerate() {
            // aw man this is so hacky but it works
            if y + 1 == log.len() && ami.sprite.current_animation() == Ami::JUMP {
                break;
            }

            let terminal_width = screen_width() - 2.0 * MARGIN;
            let target_text = format!("{},{} | ", turn.target_cell.x, turn.target_cell.y);
            let target_dims = measure_text(&target_text, Some(&font), FONT_SIZE, 1.0);
            let y_spacing = (y + 1) as f32 * (target_dims.height + MARGIN) + terminal_position;
            let x_spacing = (terminal_width - target_dims.width) / turn.distances.len() as f32;

            draw_text_ex(&target_text, 2.0 * MARGIN, y_spacing, text_params.clone());

            for (x, distance) in turn.distances.iter().enumerate() {
                if distance.is_none() {
                    continue;
                }
                draw_text_ex(
                    distance.as_ref().unwrap(),
                    x as f32 * x_spacing + target_dims.width + 2.0 * MARGIN,
                    y_spacing,
                    text_params.clone(),
                );
            }
        }

        match state {
            GameState::MainMenu => {
                // Lights on
                draw_rectangle(
                    MARGIN,
                    BAR_SIZE + MARGIN,
                    screen_width() - 2.0 * MARGIN,
                    screen_width() - 2.0 * MARGIN,
                    WHITE,
                );

                draw_texture_ex(
                    &bucko_texture,
                    (screen_width() - 64.0) / 2.0,
                    terminal_position,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(bucko_sprite.frame().dest_size),
                        source: Some(bucko_sprite.frame().source_rect),
                        ..Default::default()
                    },
                );

                let b_amount_text = bucko_amount.to_string();
                let b_amount_dims = measure_text(&b_amount_text, Some(&font), 36, 1.0);
                draw_text_ex(
                    &b_amount_text,
                    (screen_width() - b_amount_dims.width) / 2.0,
                    terminal_position + bucko_texture.height() + b_amount_dims.height,
                    TextParams {
                        font: Some(&font),
                        font_size: 36,
                        color: FORE_COLOR,
                        ..Default::default()
                    },
                );

                root_ui().push_skin(&ui_skin);
                if widgets::Button::new("<")
                    .size(Vec2::splat(36.0))
                    .position(vec2(
                        screen_width() / 2.0 - 36.0,
                        terminal_position + bucko_texture.height() + b_amount_dims.height,
                    ))
                    .ui(&mut root_ui())
                {
                    bucko_amount -= 1;
                    titlebuckos.remove();
                    // schedule_update();
                }

                if widgets::Button::new(">")
                    .size(Vec2::splat(36.0))
                    .position(vec2(
                        screen_width() / 2.0,
                        terminal_position + bucko_texture.height() + b_amount_dims.height,
                    ))
                    .ui(&mut root_ui())
                {
                    bucko_amount += 1;
                    titlebuckos.add();
                    // schedule_update();
                }

                bucko_amount = bucko_amount.clamp(1, 8);

                if enter_button.ui(&mut root_ui())
                    || is_key_pressed(KeyCode::Enter)
                    || is_key_pressed(KeyCode::KpEnter)
                {
                    buckos = Buckos::new(bucko_amount, grid_size);

                    introtext.init(bucko_amount, grid_size);

                    play_sound(
                        &sfx_light,
                        PlaySoundParams {
                            looped: false,
                            volume: sfx_volume / 100.0,
                        },
                    );

                    state = GameState::Playing
                }

                root_ui().pop_skin();

                titlebuckos.update(get_frame_time());
                for bucko in titlebuckos.positions() {
                    draw_texture_ex(
                        &bucko_texture,
                        bucko.x - 32.0 + MARGIN,
                        bucko.y - 32.0 + MARGIN + BAR_SIZE,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(Vec2::splat(64.0)),
                            source: Some(bucko_sprite.frame().source_rect),
                            ..Default::default()
                        },
                    )
                }
            }

            GameState::Playing => {
                draw_rectangle(
                    MARGIN,
                    BAR_SIZE + MARGIN,
                    screen_width() - 2.0 * MARGIN,
                    screen_width() - 2.0 * MARGIN,
                    BLACK,
                );

                for line in 1..grid_size {
                    let spacing = line as f32 * cell_size + MARGIN;
                    // Horizontal
                    draw_line(
                        spacing,
                        BAR_SIZE + MARGIN,
                        spacing,
                        terminal_position - MARGIN,
                        1.0,
                        color_u8!(255, 255, 255, 64),
                    );
                    // Vertical
                    draw_line(
                        MARGIN,
                        spacing + BAR_SIZE,
                        screen_width() - MARGIN,
                        spacing + BAR_SIZE,
                        1.0,
                        color_u8!(255, 255, 255, 64),
                    );
                }

                root_ui().push_skin(&grid_skin);
                for row in 0..grid_size {
                    for col in 0..grid_size {
                        let coords = format!("{},{}", col, row);
                        if widgets::Button::new(coords)
                            .position(vec2(
                                col as f32 * cell_size + MARGIN,
                                row as f32 * cell_size + MARGIN + BAR_SIZE,
                            ))
                            .size(Vec2::splat(cell_size))
                            .selected(selected_cell == Some(u16vec2(col, row)))
                            .ui(&mut root_ui())
                        {
                            selected_cell = Some(u16vec2(col, row))
                        }
                    }
                }
                root_ui().pop_skin();

                if !ami.visible {
                    ami.visible = true;
                    ami.sprite.playing = true;
                    ami.sprite.set_animation(Ami::REVEAL);
                }

                if log.is_empty() {
                    let text_height = measure_text("dp", Some(&font), FONT_SIZE, 1.0).height;
                    let terminal_width = screen_width() - 4.0 * MARGIN;
                    for (i, line) in introtext
                        .get(Some(&font), terminal_width)
                        .iter()
                        .enumerate()
                    {
                        let spacing = (i + 1) as f32 * (text_height + MARGIN) + terminal_position;
                        draw_text_ex(line, 2.0 * MARGIN, spacing, text_params.clone())
                    }
                }

                let win = buckos.captured.iter().all(|&b| b);
                if (log.len() == TURNS as usize || win)
                    && ami.sprite.current_animation() == Ami::WAIT
                {
                    if win {
                        play_sound(
                            &sfx_win,
                            PlaySoundParams {
                                looped: false,
                                volume: sfx_volume / 100.0,
                            },
                        );
                    } else {
                        play_sound(
                            &sfx_gameover,
                            PlaySoundParams {
                                looped: false,
                                volume: sfx_volume / 100.0,
                            },
                        );
                    }
                    ami.sprite.set_animation(Ami::GAMEOVER);
                    ami.sprite.set_frame(0);
                    selected_cell = None;
                    state = GameState::GameOver;
                    // schedule_update();
                }

                root_ui().push_skin(&ui_skin);
                if (enter_button.ui(&mut root_ui())
                    || is_key_pressed(KeyCode::Enter)
                    || is_key_pressed(KeyCode::KpEnter))
                    && ami.sprite.current_animation() == Ami::WAIT
                {
                    if let Some(cell) = selected_cell {
                        let target = cell_size
                            * (cell.as_vec2() - vec2(grid_size as f32 / 2.0, 0.0))
                            + Vec2::splat(cell_size / 2.0);
                        // test = target;
                        ami.jump(target);

                        play_sound(
                            &sfx_jump,
                            PlaySoundParams {
                                looped: false,
                                volume: sfx_volume / 100.0,
                            },
                        );

                        let distances = buckos.distances(cell);
                        // Scup pls...
                        ami.got_bucko = distances
                            .iter()
                            .any(|s| s.as_ref() == Some(&"GOT".to_string()));

                        log.push(Turn {
                            target_cell: cell,
                            distances,
                        });

                        selected_cell = None;
                    }
                }
                root_ui().pop_skin();
            }

            GameState::GameOver => {
                draw_rectangle(
                    MARGIN,
                    BAR_SIZE + MARGIN,
                    screen_width() - 2.0 * MARGIN,
                    screen_width() - 2.0 * MARGIN,
                    WHITE,
                );

                for line in 1..grid_size {
                    let spacing = line as f32 * cell_size + MARGIN;
                    // Horizontal
                    draw_line(
                        spacing,
                        BAR_SIZE + MARGIN,
                        spacing,
                        terminal_position - MARGIN,
                        1.0,
                        color_u8!(0, 0, 0, 64),
                    );
                    // Vertical
                    draw_line(
                        MARGIN,
                        spacing + BAR_SIZE,
                        screen_width() - MARGIN,
                        spacing + BAR_SIZE,
                        1.0,
                        color_u8!(0, 0, 0, 64),
                    );
                }

                let final_text = if buckos.captured.iter().all(|&b| b) {
                    ami.visible = false;

                    draw_texture_ex(
                        &win_texture,
                        2.0 * MARGIN,
                        BAR_SIZE + MARGIN,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(Vec2::splat(screen_width() - 2.0 * MARGIN)),
                            ..Default::default()
                        },
                    );

                    format!("You got them all in {} turns!", log.len())
                } else {
                    for (i, _b) in buckos.captured.iter().enumerate().filter(|(_i, &b)| !b) {
                        let position = buckos.positions[i].as_vec2() * cell_size;
                        draw_texture_ex(
                            &bucko_texture,
                            position.x + MARGIN,
                            position.y + MARGIN + BAR_SIZE,
                            WHITE,
                            DrawTextureParams {
                                dest_size: Some(Vec2::splat(cell_size)),
                                source: Some(bucko_sprite.frame().source_rect),
                                ..Default::default()
                            },
                        )
                    }

                    format!("Sorry, that's {TURNS} tries.")
                };

                draw_text_ex(
                    &final_text,
                    2.0 * MARGIN,
                    154.0 + terminal_position,
                    text_params.clone(),
                );

                root_ui().push_skin(&ui_skin);
                if enter_button.ui(&mut root_ui())
                    || is_key_pressed(KeyCode::Enter)
                    || is_key_pressed(KeyCode::KpEnter)
                {
                    introtext.reset();
                    log.clear();
                    ami.visible = false;
                    state = GameState::MainMenu
                }
                root_ui().pop_skin();
            }
        }

        {
            root_ui().push_skin(&settings_skin);
            if widgets::Button::new(settings_texture.clone())
                .size(Vec2::splat(BAR_SIZE - MARGIN))
                .position(vec2(screen_width() - BAR_SIZE, MARGIN))
                .ui(&mut root_ui())
            {
                settings_on = true;
            }

            if settings_on {
                let screen_size = vec2(screen_width(), screen_height());
                let window_size = vec2(280.0, 90.0);
                let button_size = vec2(260.0, 20.0);
                let window_pos = (screen_size - window_size) / 2.0;
                root_ui().canvas().rect(
                    Rect::new(
                        window_pos.x - MARGIN,
                        window_pos.y - MARGIN,
                        window_size.x + 2.0 * MARGIN,
                        window_size.y + 2.0 * MARGIN,
                    ),
                    FORE_COLOR,
                    FORE_COLOR,
                );
                widgets::Window::new(hash!(), window_pos, window_size)
                    .movable(false)
                    .titlebar(false)
                    .ui(&mut root_ui(), |ui| {
                        let old = bgm_volume;
                        ui.slider(hash!("bgm"), " BGM Vol", 0f32..100f32, &mut bgm_volume);
                        ui.slider(hash!(), " SFX Vol", 0f32..100f32, &mut sfx_volume);
                        if old != bgm_volume {
                            set_sound_volume(&bgm, bgm_volume / 100.0);
                        }
                        if widgets::Button::new("Close")
                            .position(vec2(10.0, 60.0))
                            .size(button_size)
                            .ui(ui)
                        {
                            settings_on = false;
                        }
                    });
            }
            root_ui().pop_skin();
        }

        // schedule_update();
        bucko_sprite.update();
        if ami.visible {
            ami.update(&sfx_climb, sfx_volume);
            ami.draw();
        }

        next_frame().await;
    }
}
