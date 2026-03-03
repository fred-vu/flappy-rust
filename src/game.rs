use crate::audio::AudioBank;
use crate::physics;
use macroquad::prelude::*;
use macroquad::rand;

pub const SCREEN_W: f32 = 480.0;
pub const SCREEN_H: f32 = 640.0;

const GROUND_H: f32 = 80.0;
const BIRD_X: f32 = 120.0;
const BIRD_RADIUS: f32 = 12.0;
const GRAVITY: f32 = 900.0;
const FLAP_VELOCITY: f32 = -300.0;
const PIPE_WIDTH: f32 = 60.0;
const PIPE_GAP_BASE: f32 = 150.0;
const PIPE_GAP_MIN: f32 = 112.0;
const PIPE_SPEED_BASE: f32 = 140.0;
const PIPE_SPEED_MAX: f32 = 210.0;
const PIPE_SPACING_BASE: f32 = 220.0;
const PIPE_SPACING_MIN: f32 = 170.0;
const PIPE_MARGIN: f32 = 60.0;
const PIPE_COUNT: usize = 4;
const DIFFICULTY_SCORE_RANGE: f32 = 60.0;
const DIFFICULTY_SMOOTHING: f32 = 1.6;
const GAP_DELTA_EASY: f32 = 170.0;
const GAP_DELTA_HARD: f32 = 120.0;

const COLOR_SKY_DAY_TOP: Color = Color::new(0.53, 0.80, 0.92, 1.0);
const COLOR_SKY_DAY_BOTTOM: Color = Color::new(0.86, 0.94, 0.99, 1.0);
const COLOR_SKY_NIGHT_TOP: Color = Color::new(0.10, 0.13, 0.22, 1.0);
const COLOR_SKY_NIGHT_BOTTOM: Color = Color::new(0.17, 0.20, 0.30, 1.0);
const COLOR_SKY_DUSK: Color = Color::new(0.94, 0.55, 0.38, 1.0);
const COLOR_GROUND: Color = Color::new(0.38, 0.27, 0.20, 1.0);
const COLOR_GROUND_DARK: Color = Color::new(0.30, 0.21, 0.16, 1.0);
const COLOR_PIPE_LIGHT: Color = Color::new(0.49, 0.84, 0.37, 1.0);
const COLOR_PIPE_MID: Color = Color::new(0.26, 0.72, 0.28, 1.0);
const COLOR_PIPE_DARK: Color = Color::new(0.13, 0.51, 0.18, 1.0);
const COLOR_PIPE_CAP: Color = Color::new(0.36, 0.78, 0.30, 1.0);
const COLOR_PIPE_CAP_SHADOW: Color = Color::new(0.17, 0.45, 0.17, 1.0);
const COLOR_PLANE_LIGHT: Color = Color::new(0.94, 0.95, 0.97, 1.0);
const COLOR_PLANE_DARK: Color = Color::new(0.72, 0.76, 0.82, 1.0);
const COLOR_PLANE_EDGE: Color = Color::new(0.52, 0.56, 0.62, 1.0);
const COLOR_PLANE_TRAIL: Color = Color::new(0.88, 0.92, 0.98, 1.0);
const COLOR_STAR: Color = Color::new(0.98, 0.88, 0.30, 1.0);
const COLOR_STAR_CORE: Color = Color::new(1.0, 0.97, 0.68, 1.0);
const COLOR_FEATHER: Color = Color::new(0.62, 0.90, 1.0, 1.0);
const COLOR_SHIELD: Color = Color::new(0.40, 0.82, 1.0, 1.0);
const COLOR_COMBO: Color = Color::new(0.98, 0.92, 0.38, 1.0);
const COLOR_FLOW: Color = Color::new(0.68, 0.90, 1.0, 1.0);
const COLOR_SHIELD_FLASH: Color = Color::new(0.70, 0.95, 1.0, 1.0);
const COLOR_BOOST: Color = Color::new(0.98, 0.54, 0.22, 1.0);
const COLOR_BOOST_CORE: Color = Color::new(1.0, 0.86, 0.62, 1.0);

const STAR_RADIUS: f32 = 9.0;
const FEATHER_RADIUS: f32 = 10.0;
const SHIELD_RADIUS: f32 = 11.0;
const BOOST_RADIUS: f32 = 11.0;
const STAR_BASE_SCORE: u32 = 2;
const COMBO_WINDOW: f32 = 2.2;
const MAX_COMBO_BONUS: u32 = 6;
const PERFECT_ZONE: f32 = 22.0;
const MAX_FLOW: u32 = 9;
const FEATHER_DURATION: f32 = 4.5;
const SHIELD_DURATION: f32 = 5.0;
const BOOST_DURATION: f32 = 6.0;
const BOOST_MULTIPLIER: u32 = 2;
const CLOSE_MARGIN: f32 = 18.0;
const CLOSE_BONUS: u32 = 1;
const CLOSE_FLASH_DURATION: f32 = 0.7;
const CLOUD_COUNT: usize = 6;

const PLANE_PIXEL_SIZE: f32 = 2.0;
const PLANE_WIDTH: usize = 12;
const PLANE_HEIGHT: usize = 7;
const PIPE_PIXEL_SIZE: f32 = 5.0;
const PIPE_CAP_ROWS: i32 = 3;
const PIPE_CAP_OVERHANG: f32 = 7.0;
const PLANE_PIXEL_MAP: [&str; PLANE_HEIGHT] = [
    "...........l",
    ".........lll",
    "......llllll",
    "..llleeeeeee",
    "...dddddddd.",
    ".....dddddd.",
    ".......ddd..",
];

#[derive(Clone)]
struct Pipe {
    x: f32,
    gap_y: f32,
    gap: f32,
    scored: bool,
}

impl Pipe {
    fn rects(&self) -> (Rect, Rect) {
        let ground_y = SCREEN_H - GROUND_H;
        let gap_half = self.gap / 2.0;
        let top_h = (self.gap_y - gap_half).max(0.0);
        let bottom_y = self.gap_y + gap_half;
        let bottom_h = (ground_y - bottom_y).max(0.0);

        let top = Rect::new(self.x, 0.0, PIPE_WIDTH, top_h);
        let bottom = Rect::new(self.x, bottom_y, PIPE_WIDTH, bottom_h);
        (top, bottom)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum CollectibleKind {
    Star,
    Feather,
    Shield,
    Boost,
}

#[derive(Clone)]
struct Collectible {
    x: f32,
    y: f32,
    kind: CollectibleKind,
    radius: f32,
    bob_offset: f32,
    collected: bool,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum PowerKind {
    Feather,
    Shield,
}

#[derive(Clone)]
struct Cloud {
    x: f32,
    y: f32,
    speed: f32,
    scale: f32,
}

pub struct Game {
    bird_y: f32,
    bird_vy: f32,
    pipes: Vec<Pipe>,
    collectibles: Vec<Collectible>,
    clouds: Vec<Cloud>,
    score: u32,
    difficulty: f32,
    combo_count: u32,
    combo_timer: f32,
    flow: u32,
    flow_flash: f32,
    power_kind: Option<PowerKind>,
    power_timer: f32,
    boost_timer: f32,
    close_timer: f32,
    shield_flash: f32,
    ground_scroll: f32,
    game_over: bool,
    started: bool,
    paused: bool,
    muted: bool,
    plane_pitch: f32,
    audio: AudioBank,
}

impl Game {
    pub fn new(audio: AudioBank) -> Self {
        let mut game = Self {
            bird_y: SCREEN_H / 2.0,
            bird_vy: 0.0,
            pipes: Vec::new(),
            collectibles: Vec::new(),
            clouds: Vec::new(),
            score: 0,
            difficulty: 0.0,
            combo_count: 0,
            combo_timer: 0.0,
            flow: 0,
            flow_flash: 0.0,
            power_kind: None,
            power_timer: 0.0,
            boost_timer: 0.0,
            close_timer: 0.0,
            shield_flash: 0.0,
            ground_scroll: 0.0,
            game_over: false,
            started: false,
            paused: false,
            muted: false,
            plane_pitch: 0.0,
            audio,
        };
        game.reset();
        game
    }

    pub fn update(&mut self, dt: f32) {
        if is_key_pressed(KeyCode::M) {
            self.muted = !self.muted;
            self.audio.set_muted(self.muted);
        }

        if is_key_pressed(KeyCode::P) && !self.game_over {
            self.paused = !self.paused;
        }

        if is_key_pressed(KeyCode::R) && self.game_over {
            self.reset();
            return;
        }

        if self.paused {
            return;
        }

        self.update_clouds(dt);
        self.update_timers(dt);

        let flap = is_key_pressed(KeyCode::Space)
            || is_key_pressed(KeyCode::Up)
            || is_mouse_button_pressed(MouseButton::Left);

        let mut play_flap = false;
        let mut play_score = false;
        let mut play_game_over = false;
        let mut pending_score: u32 = 0;

        let (gravity, flap_velocity, max_fall) = if self.power_kind == Some(PowerKind::Feather) {
            (GRAVITY * 0.55, FLAP_VELOCITY * 0.9, 320.0)
        } else {
            (GRAVITY, FLAP_VELOCITY, 520.0)
        };

        if !self.started {
            self.plane_pitch = lerp(self.plane_pitch, 0.0, (dt * 7.0).clamp(0.0, 1.0));
            if flap {
                self.started = true;
                self.bird_vy = flap_velocity;
                play_flap = true;
            }
            if play_flap {
                self.audio.play_flap();
            }
            return;
        }

        if self.game_over {
            return;
        }

        self.update_difficulty(dt);
        let pipe_speed = self.current_pipe_speed();

        if flap {
            self.bird_vy = flap_velocity;
            play_flap = true;
        }

        self.bird_vy += gravity * dt;
        if self.bird_vy > max_fall {
            self.bird_vy = max_fall;
        }
        let target_pitch = (self.bird_vy / 480.0).clamp(-0.55, 0.65);
        self.plane_pitch = lerp(self.plane_pitch, target_pitch, (dt * 8.0).clamp(0.0, 1.0));
        self.bird_y += self.bird_vy * dt;

        let ground_y = SCREEN_H - GROUND_H;
        if self.bird_y - BIRD_RADIUS < 0.0 {
            self.bird_y = BIRD_RADIUS;
            self.bird_vy = 0.0;
        }

        if self.bird_y + BIRD_RADIUS >= ground_y {
            self.bird_y = ground_y - BIRD_RADIUS;
            self.game_over = true;
            play_game_over = true;
        }

        self.ground_scroll = (self.ground_scroll + pipe_speed * dt) % 40.0;

        for pipe in &mut self.pipes {
            pipe.x -= pipe_speed * dt;
        }

        for collectible in &mut self.collectibles {
            collectible.x -= pipe_speed * dt;
        }

        self.pipes.retain(|pipe| pipe.x + PIPE_WIDTH > 0.0);
        self.ensure_pipes();

        if !self.game_over {
            let mut bird_pos = vec2(BIRD_X, self.bird_y);
            for pipe in &mut self.pipes {
                let (top, bottom) = pipe.rects();
                let collided = physics::circle_rect_collision(bird_pos, BIRD_RADIUS, top)
                    || physics::circle_rect_collision(bird_pos, BIRD_RADIUS, bottom);
                if collided {
                    if self.power_kind == Some(PowerKind::Shield) {
                        self.power_kind = None;
                        self.power_timer = 0.0;
                        self.shield_flash = 0.35;
                        self.bird_y = pipe.gap_y;
                        self.bird_vy = 0.0;
                        bird_pos.y = self.bird_y;
                    } else {
                        self.game_over = true;
                        play_game_over = true;
                        break;
                    }
                }

                if !pipe.scored && pipe.x + PIPE_WIDTH < BIRD_X {
                    pipe.scored = true;
                    let distance = (self.bird_y - pipe.gap_y).abs();
                    let perfect = distance <= PERFECT_ZONE;
                    let close_zone = (pipe.gap * 0.5 - CLOSE_MARGIN).max(PERFECT_ZONE + 8.0);
                    let close_call = distance >= close_zone && !perfect;
                    if perfect {
                        self.flow = (self.flow + 1).min(MAX_FLOW);
                        self.flow_flash = 0.6;
                    } else {
                        self.flow = 0;
                    }
                    let flow_bonus = if perfect { 1 + (self.flow / 3) } else { 0 };
                    let multiplier = if self.boost_timer > 0.0 {
                        BOOST_MULTIPLIER
                    } else {
                        1
                    };
                    let points = (1 + flow_bonus).saturating_mul(multiplier);
                    pending_score = pending_score.saturating_add(points);
                    if close_call {
                        let close_points = CLOSE_BONUS.saturating_mul(multiplier);
                        pending_score = pending_score.saturating_add(close_points);
                        self.close_timer = CLOSE_FLASH_DURATION;
                    }
                    play_score = true;
                }
            }
        }

        if !self.game_over {
            let time = get_time() as f32;
            let bird_pos = vec2(BIRD_X, self.bird_y);
            for collectible in &mut self.collectibles {
                if collectible.collected {
                    continue;
                }

                let bob = (time * 3.2 + collectible.bob_offset).sin() * 4.0;
                let item_pos = vec2(collectible.x, collectible.y + bob);
                let radius = BIRD_RADIUS + collectible.radius;
                let dist_sq = (bird_pos.x - item_pos.x).powi(2) + (bird_pos.y - item_pos.y).powi(2);

                if dist_sq <= radius * radius {
                    collectible.collected = true;
                    match collectible.kind {
                        CollectibleKind::Star => {
                            if self.combo_timer > 0.0 {
                                self.combo_count = self.combo_count.saturating_add(1);
                            } else {
                                self.combo_count = 1;
                            }
                            self.combo_timer = COMBO_WINDOW;
                            let combo_bonus = self.combo_count.min(MAX_COMBO_BONUS);
                            let bonus = STAR_BASE_SCORE + combo_bonus;
                            let multiplier = if self.boost_timer > 0.0 {
                                BOOST_MULTIPLIER
                            } else {
                                1
                            };
                            let points = bonus.saturating_mul(multiplier);
                            pending_score = pending_score.saturating_add(points);
                            play_score = true;
                        }
                        CollectibleKind::Feather => {
                            self.power_kind = Some(PowerKind::Feather);
                            self.power_timer = FEATHER_DURATION;
                            let multiplier = if self.boost_timer > 0.0 {
                                BOOST_MULTIPLIER
                            } else {
                                1
                            };
                            let points = 1u32.saturating_mul(multiplier);
                            pending_score = pending_score.saturating_add(points);
                            play_score = true;
                        }
                        CollectibleKind::Shield => {
                            self.power_kind = Some(PowerKind::Shield);
                            self.power_timer = SHIELD_DURATION;
                            let multiplier = if self.boost_timer > 0.0 {
                                BOOST_MULTIPLIER
                            } else {
                                1
                            };
                            let points = 1u32.saturating_mul(multiplier);
                            pending_score = pending_score.saturating_add(points);
                            play_score = true;
                        }
                        CollectibleKind::Boost => {
                            self.boost_timer = BOOST_DURATION;
                            let multiplier = if self.boost_timer > 0.0 {
                                BOOST_MULTIPLIER
                            } else {
                                1
                            };
                            let points = 1u32.saturating_mul(multiplier);
                            pending_score = pending_score.saturating_add(points);
                            play_score = true;
                        }
                    }
                }
            }
        }

        self.collectibles.retain(|collectible| {
            !collectible.collected && collectible.x + collectible.radius > 0.0
        });

        if pending_score > 0 {
            self.score = self.score.saturating_add(pending_score);
        }

        if play_flap {
            if !self.muted {
                self.audio.play_flap();
            }
        }
        if play_score {
            if !self.muted {
                self.audio.play_score();
            }
        }
        if play_game_over {
            if !self.muted {
                self.audio.play_game_over();
            }
        }
    }

    pub fn draw(&self) {
        let time = get_time() as f32;
        self.draw_background(time);
        self.draw_clouds(time);
        self.draw_boost_overlay(time);
        self.draw_speed_lines(time);

        for pipe in &self.pipes {
            let (top, bottom) = pipe.rects();
            self.draw_pipe_pixel_art(top, true, time);
            self.draw_pipe_pixel_art(bottom, false, time);
        }

        for collectible in &self.collectibles {
            self.draw_collectible(collectible, time);
        }

        let bird_pos = vec2(BIRD_X, self.bird_y);
        self.draw_power_aura(bird_pos, time);
        self.draw_plane(bird_pos, time);

        let ground_y = SCREEN_H - GROUND_H;
        self.draw_ground(ground_y);

        let score_text = format!("{}", self.score);
        let font_size = 36.0;
        let score_measure = measure_text(&score_text, None, font_size as u16, 1.0);
        draw_text(
            &score_text,
            (SCREEN_W - score_measure.width) / 2.0,
            50.0,
            font_size,
            WHITE,
        );

        if self.combo_count >= 2 && self.combo_timer > 0.0 {
            let combo_text = format!("Combo x{}", self.combo_count);
            draw_text(&combo_text, 16.0, 42.0, 20.0, COLOR_COMBO);
        }

        if self.flow > 0 {
            let flow_burst = (self.flow_flash / 0.6).clamp(0.0, 1.0);
            let flow_size = 18.0 + 6.0 * flow_burst;
            let flow_text = format!("Flow x{}", self.flow);
            draw_text(&flow_text, 16.0, 68.0, flow_size, COLOR_FLOW);
        }

        if self.close_timer > 0.0 {
            let t = (self.close_timer / CLOSE_FLASH_DURATION).clamp(0.0, 1.0);
            let size = 18.0 + 6.0 * t;
            draw_text("Close +1", 16.0, 94.0, size, COLOR_BOOST);
        }

        if let Some(power_kind) = self.power_kind {
            let (label, color, total) = match power_kind {
                PowerKind::Feather => ("Feather", COLOR_FEATHER, FEATHER_DURATION),
                PowerKind::Shield => ("Shield", COLOR_SHIELD, SHIELD_DURATION),
            };
            let power_text = format!("{} {:.1}s", label, self.power_timer.max(0.0));
            let power_size = 18.0;
            let power_measure = measure_text(&power_text, None, power_size as u16, 1.0);
            let x = SCREEN_W - power_measure.width - 16.0;
            let y = 34.0;
            draw_text(&power_text, x, y, power_size, color);

            let bar_w = 90.0;
            let bar_h = 4.0;
            let fill = (self.power_timer / total).clamp(0.0, 1.0);
            draw_rectangle(x, y + 6.0, bar_w, bar_h, Color::new(1.0, 1.0, 1.0, 0.18));
            draw_rectangle(x, y + 6.0, bar_w * fill, bar_h, color);
        }

        if self.boost_timer > 0.0 {
            let boost_text = format!("Boost x{}", BOOST_MULTIPLIER);
            let boost_size = 18.0;
            let boost_measure = measure_text(&boost_text, None, boost_size as u16, 1.0);
            let x = SCREEN_W - boost_measure.width - 16.0;
            let y = if self.power_kind.is_some() {
                58.0
            } else {
                34.0
            };
            draw_text(&boost_text, x, y, boost_size, COLOR_BOOST);

            let bar_w = 90.0;
            let bar_h = 4.0;
            let fill = (self.boost_timer / BOOST_DURATION).clamp(0.0, 1.0);
            draw_rectangle(x, y + 6.0, bar_w, bar_h, Color::new(1.0, 1.0, 1.0, 0.18));
            draw_rectangle(x, y + 6.0, bar_w * fill, bar_h, COLOR_BOOST);
        }

        self.draw_status_hints();

        if !self.started {
            self.draw_centered_text("Press Space or Click to Flap", SCREEN_H / 2.0, 28.0);
            self.draw_centered_text(
                "Collect stars, boost orbs, and power-ups",
                SCREEN_H / 2.0 + 34.0,
                22.0,
            );
            self.draw_centered_text("Perfect gaps build Flow", SCREEN_H / 2.0 + 60.0, 20.0);
            self.draw_centered_text("Close calls score extra", SCREEN_H / 2.0 + 84.0, 18.0);
        }

        if self.paused {
            self.draw_centered_text("Paused", SCREEN_H / 2.0 - 24.0, 40.0);
            self.draw_centered_text("Press P to Resume", SCREEN_H / 2.0 + 18.0, 22.0);
        }

        if self.game_over {
            self.draw_centered_text("Game Over", SCREEN_H / 2.0 - 20.0, 40.0);
            self.draw_centered_text("Press R to Restart", SCREEN_H / 2.0 + 24.0, 24.0);
        }
    }

    fn draw_centered_text(&self, text: &str, y: f32, size: f32) {
        let measure = measure_text(text, None, size as u16, 1.0);
        draw_text(text, (SCREEN_W - measure.width) / 2.0, y, size, WHITE);
    }

    fn draw_status_hints(&self) {
        let ground_y = SCREEN_H - GROUND_H;
        let hint_size = 18.0;
        let hint_color = Color::new(1.0, 1.0, 1.0, 0.85);
        let x = 16.0;
        let pause_text = if self.paused { "P: Resume" } else { "P: Pause" };
        let mute_text = if self.muted { "M: Unmute" } else { "M: Mute" };
        let mute_color = if self.muted { COLOR_BOOST } else { hint_color };

        draw_text(pause_text, x, ground_y + 28.0, hint_size, hint_color);
        draw_text(mute_text, x, ground_y + 50.0, hint_size, mute_color);
    }

    fn draw_plane(&self, position: Vec2, time: f32) {
        let wobble = if self.started {
            (time * 2.2).sin() * 0.035
        } else {
            (time * 2.2).sin() * 0.06
        };
        // Forward heading stays right-facing while pitch follows vertical movement.
        let angle = self.plane_pitch + wobble;

        let forward = vec2(angle.cos(), angle.sin());
        self.draw_plane_trail(position, forward, time);

        let center = vec2(
            PLANE_WIDTH as f32 * PLANE_PIXEL_SIZE * 0.5,
            PLANE_HEIGHT as f32 * PLANE_PIXEL_SIZE * 0.5,
        );

        for (row, line) in PLANE_PIXEL_MAP.iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                let color = match ch {
                    'l' => Some(COLOR_PLANE_LIGHT),
                    'd' => Some(COLOR_PLANE_DARK),
                    'e' => Some(COLOR_PLANE_EDGE),
                    _ => None,
                };

                if let Some(color) = color {
                    let local = vec2(
                        (col as f32 + 0.5) * PLANE_PIXEL_SIZE,
                        (row as f32 + 0.5) * PLANE_PIXEL_SIZE,
                    ) - center;
                    let rotated = rotate_vec(local, angle);
                    let pixel_center = position + rotated;
                    draw_rectangle(
                        pixel_center.x - PLANE_PIXEL_SIZE * 0.5,
                        pixel_center.y - PLANE_PIXEL_SIZE * 0.5,
                        PLANE_PIXEL_SIZE,
                        PLANE_PIXEL_SIZE,
                        color,
                    );
                }
            }
        }
    }

    fn draw_pipe_pixel_art(&self, rect: Rect, top: bool, time: f32) {
        if rect.w <= 0.0 || rect.h <= 0.0 {
            return;
        }

        let cols = (rect.w / PIPE_PIXEL_SIZE).ceil() as i32;
        let rows = (rect.h / PIPE_PIXEL_SIZE).ceil() as i32;
        let max_col = cols.saturating_sub(1);
        let segment_rows = 6;
        let segment_offset = (rect.y / PIPE_PIXEL_SIZE).floor() as i32;
        let highlight_col = ((cols as f32) * 0.28).round() as i32;
        let shadow_col = ((cols as f32) * 0.78).round() as i32;
        let highlight_col = highlight_col.clamp(0, max_col);
        let shadow_col = shadow_col.clamp(0, max_col);
        let rim_light = lerp_color(COLOR_PIPE_LIGHT, WHITE, 0.45);
        let rim_glow = lerp_color(COLOR_PIPE_LIGHT, WHITE, 0.22);
        let groove = lerp_color(COLOR_PIPE_DARK, BLACK, 0.25);
        let deep_shadow = lerp_color(COLOR_PIPE_DARK, BLACK, 0.55);
        for row in 0..rows {
            for col in 0..cols {
                let x = rect.x + col as f32 * PIPE_PIXEL_SIZE;
                let y = rect.y + row as f32 * PIPE_PIXEL_SIZE;
                let w = (rect.x + rect.w - x).min(PIPE_PIXEL_SIZE).max(0.0);
                let h = (rect.y + rect.h - y).min(PIPE_PIXEL_SIZE).max(0.0);
                if w <= 0.0 || h <= 0.0 {
                    continue;
                }

                let mut color = if col <= 1 {
                    COLOR_PIPE_LIGHT
                } else if col >= cols.saturating_sub(2) {
                    COLOR_PIPE_DARK
                } else {
                    COLOR_PIPE_MID
                };

                if col == highlight_col {
                    color = lerp_color(color, rim_light, 0.7);
                } else if col == highlight_col + 1 {
                    color = lerp_color(color, rim_glow, 0.45);
                } else if col == shadow_col {
                    color = lerp_color(color, deep_shadow, 0.35);
                }

                let segment_row = (row + segment_offset).rem_euclid(segment_rows);
                if segment_row == 0 {
                    color = lerp_color(color, WHITE, 0.18);
                } else if segment_row == segment_rows - 1 {
                    color = lerp_color(color, groove, 0.65);
                }

                let shimmer = (time * 0.6).floor();
                let noise = pseudo_rand(row as f32 * 12.7 + col as f32 * 7.3 + shimmer * 1.37);
                if noise > 0.985 {
                    color = lerp_color(color, WHITE, 0.12);
                } else if noise < 0.015 {
                    color = lerp_color(color, BLACK, 0.08);
                }

                draw_rectangle(x, y, w, h, color);
            }
        }

        let cap_h = (PIPE_CAP_ROWS as f32 * PIPE_PIXEL_SIZE).min(rect.h.max(PIPE_PIXEL_SIZE));
        let cap_y = if top { rect.y + rect.h - cap_h } else { rect.y };
        let cap_x = rect.x - PIPE_CAP_OVERHANG;
        let cap_w = rect.w + PIPE_CAP_OVERHANG * 2.0;
        let cap_cols = (cap_w / PIPE_PIXEL_SIZE).ceil() as i32;
        let cap_rows = (cap_h / PIPE_PIXEL_SIZE).ceil() as i32;
        let cap_max_col = cap_cols.saturating_sub(1);
        let cap_highlight_col = ((cap_cols as f32) * 0.28).round() as i32;
        let cap_shadow_col = ((cap_cols as f32) * 0.78).round() as i32;
        let cap_highlight_col = cap_highlight_col.clamp(0, cap_max_col);
        let cap_shadow_col = cap_shadow_col.clamp(0, cap_max_col);
        let cap_highlight = lerp_color(COLOR_PIPE_CAP, WHITE, 0.35);
        let cap_shadow = lerp_color(COLOR_PIPE_CAP_SHADOW, BLACK, 0.35);

        for row in 0..cap_rows {
            for col in 0..cap_cols {
                let x = cap_x + col as f32 * PIPE_PIXEL_SIZE;
                let y = cap_y + row as f32 * PIPE_PIXEL_SIZE;
                let w = (cap_x + cap_w - x).min(PIPE_PIXEL_SIZE).max(0.0);
                let h = (cap_y + cap_h - y).min(PIPE_PIXEL_SIZE).max(0.0);
                if w <= 0.0 || h <= 0.0 {
                    continue;
                }

                let mut color = if col <= 1 {
                    lerp_color(COLOR_PIPE_CAP, WHITE, 0.12)
                } else if col >= cap_cols.saturating_sub(2) {
                    COLOR_PIPE_CAP_SHADOW
                } else {
                    COLOR_PIPE_CAP
                };

                if row == 0 {
                    color = lerp_color(color, cap_highlight, 0.7);
                } else if row == cap_rows - 1 {
                    color = lerp_color(color, cap_shadow, 0.7);
                }

                if col == cap_highlight_col {
                    color = lerp_color(color, WHITE, 0.18);
                } else if col == cap_shadow_col {
                    color = lerp_color(color, BLACK, 0.12);
                }

                draw_rectangle(x, y, w, h, color);
            }
        }

        let seam_y = if top {
            cap_y - PIPE_PIXEL_SIZE
        } else {
            cap_y + cap_h
        };
        if seam_y >= rect.y && seam_y + PIPE_PIXEL_SIZE <= rect.y + rect.h {
            draw_rectangle(
                rect.x,
                seam_y,
                rect.w,
                PIPE_PIXEL_SIZE,
                with_alpha(BLACK, 0.12),
            );
            draw_rectangle(
                rect.x,
                seam_y + PIPE_PIXEL_SIZE * 0.2,
                rect.w,
                PIPE_PIXEL_SIZE * 0.2,
                with_alpha(WHITE, 0.08),
            );
        }

        let rivet_size = (PIPE_PIXEL_SIZE - 1.0).max(2.0);
        let rivet_y = if top {
            cap_y + PIPE_PIXEL_SIZE
        } else {
            cap_y + cap_h - PIPE_PIXEL_SIZE - rivet_size
        };
        let left_rivet_x = cap_x + PIPE_PIXEL_SIZE * 1.2;
        let right_rivet_x = cap_x + cap_w - PIPE_PIXEL_SIZE * 1.2 - rivet_size;
        draw_rectangle(
            left_rivet_x,
            rivet_y,
            rivet_size,
            rivet_size,
            COLOR_PIPE_DARK,
        );
        draw_rectangle(
            right_rivet_x,
            rivet_y,
            rivet_size,
            rivet_size,
            COLOR_PIPE_DARK,
        );
    }

    fn draw_plane_trail(&self, position: Vec2, forward: Vec2, time: f32) {
        let active_strength = if self.started && !self.game_over {
            1.0
        } else {
            0.45
        };
        let flutter = (time * 8.0).sin() * PLANE_PIXEL_SIZE * 0.4;
        let side = vec2(-forward.y, forward.x);

        for i in 0..3 {
            let step = i as f32 + 1.0;
            let offset = -forward * (PLANE_PIXEL_SIZE * (3.0 + step * 2.1))
                + side * (flutter * (1.0 - step * 0.18));
            let size = PLANE_PIXEL_SIZE * (1.6 - step * 0.25);
            let alpha = 0.35 * active_strength * (1.0 - step * 0.2);
            let color = Color::new(
                COLOR_PLANE_TRAIL.r,
                COLOR_PLANE_TRAIL.g,
                COLOR_PLANE_TRAIL.b,
                alpha.clamp(0.05, 0.35),
            );

            draw_rectangle(
                position.x + offset.x - size * 0.5,
                position.y + offset.y - size * 0.5,
                size,
                size,
                color,
            );
        }
    }

    fn update_timers(&mut self, dt: f32) {
        if self.combo_timer > 0.0 {
            self.combo_timer -= dt;
            if self.combo_timer <= 0.0 {
                self.combo_timer = 0.0;
                self.combo_count = 0;
            }
        }

        if self.flow_flash > 0.0 {
            self.flow_flash = (self.flow_flash - dt).max(0.0);
        }

        if self.power_timer > 0.0 {
            self.power_timer -= dt;
            if self.power_timer <= 0.0 {
                self.power_timer = 0.0;
                self.power_kind = None;
            }
        }

        if self.boost_timer > 0.0 {
            self.boost_timer = (self.boost_timer - dt).max(0.0);
        }

        if self.close_timer > 0.0 {
            self.close_timer = (self.close_timer - dt).max(0.0);
        }

        if self.shield_flash > 0.0 {
            self.shield_flash = (self.shield_flash - dt).max(0.0);
        }
    }

    fn update_clouds(&mut self, dt: f32) {
        for cloud in &mut self.clouds {
            cloud.x -= cloud.speed * dt;
        }

        let max_y = (SCREEN_H - GROUND_H - 140.0).max(80.0);
        for cloud in &mut self.clouds {
            let threshold = -140.0 * cloud.scale;
            if cloud.x < threshold {
                cloud.x = SCREEN_W + rand::gen_range(40.0, 160.0);
                cloud.y = rand::gen_range(40.0, max_y);
                cloud.scale = rand::gen_range(0.6, 1.35);
                cloud.speed = rand::gen_range(18.0, 42.0) * (0.7 + cloud.scale * 0.3);
            }
        }
    }

    fn update_difficulty(&mut self, dt: f32) {
        let target = (self.score as f32 / DIFFICULTY_SCORE_RANGE).clamp(0.0, 1.0);
        if (self.difficulty - target).abs() < f32::EPSILON {
            return;
        }

        let smoothing = 1.0 - (-DIFFICULTY_SMOOTHING * dt).exp();
        self.difficulty = lerp(self.difficulty, target, smoothing).clamp(0.0, 1.0);
    }

    fn current_pipe_speed(&self) -> f32 {
        lerp(PIPE_SPEED_BASE, PIPE_SPEED_MAX, self.difficulty)
    }

    fn current_pipe_spacing(&self) -> f32 {
        lerp(PIPE_SPACING_BASE, PIPE_SPACING_MIN, self.difficulty)
    }

    fn current_pipe_gap(&self) -> f32 {
        lerp(PIPE_GAP_BASE, PIPE_GAP_MIN, self.difficulty)
    }

    fn rightmost_gap_y(&self) -> Option<f32> {
        let mut rightmost: Option<&Pipe> = None;
        for pipe in &self.pipes {
            if rightmost.map_or(true, |current| pipe.x > current.x) {
                rightmost = Some(pipe);
            }
        }
        rightmost.map(|pipe| pipe.gap_y)
    }

    fn max_gap_delta(&self) -> f32 {
        lerp(GAP_DELTA_EASY, GAP_DELTA_HARD, self.difficulty)
    }

    fn draw_background(&self, time: f32) {
        let (top, bottom, night_strength, day_mix) = sky_palette(time, self.flow);
        let bands = 28;
        let band_h = SCREEN_H / bands as f32;
        for i in 0..bands {
            let t = i as f32 / (bands - 1) as f32;
            let color = lerp_color(top, bottom, t);
            draw_rectangle(0.0, i as f32 * band_h, SCREEN_W, band_h + 1.0, color);
        }

        let sun_x = SCREEN_W * 0.78;
        let sun_y = 110.0 + 40.0 * (1.0 - day_mix);
        let sun_alpha = 0.12 + 0.38 * day_mix;
        draw_circle(
            sun_x,
            sun_y,
            38.0,
            with_alpha(Color::new(1.0, 0.92, 0.65, 1.0), sun_alpha),
        );
        draw_circle(
            sun_x,
            sun_y,
            20.0,
            with_alpha(Color::new(1.0, 0.98, 0.82, 1.0), sun_alpha + 0.2),
        );

        let moon_x = SCREEN_W * 0.22;
        let moon_y = 130.0 + 30.0 * day_mix;
        let moon_alpha = 0.12 + 0.3 * night_strength;
        let moon_color = with_alpha(Color::new(0.90, 0.95, 1.0, 1.0), moon_alpha);
        draw_circle(moon_x, moon_y, 22.0, moon_color);
        draw_circle(
            moon_x + 8.0,
            moon_y - 4.0,
            18.0,
            with_alpha(lerp_color(top, bottom, 0.15), moon_alpha),
        );

        if night_strength > 0.08 {
            let night = night_strength.powf(1.2);
            for i in 0..34 {
                let seed = i as f32 + 1.0;
                let x = pseudo_rand(seed * 12.9898) * SCREEN_W;
                let y = pseudo_rand(seed * 78.233) * (SCREEN_H - GROUND_H - 140.0) + 40.0;
                let twinkle = (time * 1.6 + seed * 3.4).sin() * 0.5 + 0.5;
                let alpha = (0.12 + 0.5 * twinkle) * night;
                let radius = 0.9 + 1.6 * twinkle;
                draw_circle(x, y, radius, with_alpha(COLOR_STAR, alpha));
                draw_circle(
                    x,
                    y,
                    radius * 0.45,
                    with_alpha(COLOR_STAR_CORE, alpha.min(0.9)),
                );
            }
        }
    }

    fn draw_clouds(&self, time: f32) {
        if self.clouds.is_empty() {
            return;
        }
        let (_, _, night_strength, day_mix) = sky_palette(time, self.flow);
        let cloud_alpha = lerp(0.12, 0.36, day_mix) * (1.0 - night_strength * 0.4);
        let cloud_color = Color::new(1.0, 1.0, 1.0, cloud_alpha.clamp(0.08, 0.4));

        for cloud in &self.clouds {
            let x = cloud.x;
            let y = cloud.y;
            let s = cloud.scale;
            draw_circle(x, y, 18.0 * s, cloud_color);
            draw_circle(x + 16.0 * s, y - 6.0 * s, 16.0 * s, cloud_color);
            draw_circle(x + 34.0 * s, y, 20.0 * s, cloud_color);
            draw_rectangle(x - 10.0 * s, y, 52.0 * s, 16.0 * s, cloud_color);
        }
    }

    fn draw_boost_overlay(&self, time: f32) {
        if self.boost_timer <= 0.0 {
            return;
        }
        let strength = (self.boost_timer / BOOST_DURATION).clamp(0.0, 1.0);
        let pulse = (time * 2.4).sin() * 0.5 + 0.5;
        let alpha = (0.04 + 0.06 * pulse) * strength;
        draw_rectangle(
            0.0,
            0.0,
            SCREEN_W,
            SCREEN_H - GROUND_H,
            with_alpha(COLOR_BOOST, alpha),
        );
    }

    fn draw_speed_lines(&self, time: f32) {
        if self.boost_timer <= 0.0 {
            return;
        }
        let strength = (self.boost_timer / BOOST_DURATION).clamp(0.0, 1.0);
        let travel = time * (260.0 + 140.0 * strength);
        let max_y = SCREEN_H - GROUND_H - 60.0;

        for i in 0..12 {
            let seed = i as f32 + 1.0;
            let y = 40.0 + pseudo_rand(seed * 19.31) * max_y;
            let offset = (travel + seed * 120.0) % (SCREEN_W + 160.0);
            let x = SCREEN_W - offset;
            let len = 20.0 + pseudo_rand(seed * 7.77) * 40.0;
            let flicker = (time * 3.2 + seed).sin().abs();
            let alpha = (0.12 + 0.18 * strength) * (0.6 + 0.4 * flicker);
            draw_line(x, y, x + len, y, 1.4, with_alpha(COLOR_BOOST_CORE, alpha));
        }
    }

    fn draw_collectible(&self, collectible: &Collectible, time: f32) {
        if collectible.collected {
            return;
        }

        let bob = (time * 3.2 + collectible.bob_offset).sin() * 4.0;
        let x = collectible.x;
        let y = collectible.y + bob;

        match collectible.kind {
            CollectibleKind::Star => {
                let pulse = (time * 4.4 + collectible.bob_offset).sin() * 0.5 + 0.5;
                let glow = STAR_RADIUS * (1.5 + 0.2 * pulse);
                draw_circle(x, y, glow, with_alpha(COLOR_STAR, 0.18));
                draw_circle(x, y, STAR_RADIUS, COLOR_STAR);
                draw_circle(x, y, STAR_RADIUS * 0.55, COLOR_STAR_CORE);
                draw_line(
                    x - STAR_RADIUS,
                    y,
                    x + STAR_RADIUS,
                    y,
                    1.4,
                    with_alpha(COLOR_STAR_CORE, 0.8),
                );
                draw_line(
                    x,
                    y - STAR_RADIUS,
                    x,
                    y + STAR_RADIUS,
                    1.4,
                    with_alpha(COLOR_STAR_CORE, 0.8),
                );
            }
            CollectibleKind::Feather => {
                let pulse = (time * 5.0 + collectible.bob_offset).sin() * 0.5 + 0.5;
                let radius = FEATHER_RADIUS;
                draw_circle(
                    x,
                    y,
                    radius * (1.4 + 0.2 * pulse),
                    with_alpha(COLOR_FEATHER, 0.16),
                );
                draw_circle(x, y, radius, COLOR_FEATHER);
                draw_line(
                    x - radius * 0.7,
                    y,
                    x + radius * 0.6,
                    y + radius * 0.35,
                    1.5,
                    with_alpha(WHITE, 0.7),
                );
                draw_line(
                    x - radius * 0.25,
                    y - radius * 0.55,
                    x + radius * 0.25,
                    y + radius * 0.6,
                    1.2,
                    with_alpha(WHITE, 0.6),
                );
            }
            CollectibleKind::Shield => {
                let pulse = (time * 4.2 + collectible.bob_offset).sin() * 0.5 + 0.5;
                let radius = SHIELD_RADIUS;
                draw_circle(
                    x,
                    y,
                    radius * (1.5 + 0.2 * pulse),
                    with_alpha(COLOR_SHIELD, 0.2),
                );
                draw_circle(x, y, radius, with_alpha(COLOR_SHIELD, 0.9));
                draw_circle_lines(x, y, radius, 2.0, with_alpha(WHITE, 0.55));
                draw_line(
                    x,
                    y - radius * 0.55,
                    x,
                    y + radius * 0.55,
                    1.3,
                    with_alpha(WHITE, 0.6),
                );
            }
            CollectibleKind::Boost => {
                let pulse = (time * 5.6 + collectible.bob_offset).sin() * 0.5 + 0.5;
                let radius = BOOST_RADIUS;
                draw_circle(
                    x,
                    y,
                    radius * (1.6 + 0.25 * pulse),
                    with_alpha(COLOR_BOOST, 0.18),
                );
                draw_circle(x, y, radius, COLOR_BOOST);
                draw_circle(x, y, radius * 0.55, COLOR_BOOST_CORE);
                draw_line(
                    x - radius * 0.6,
                    y + radius * 0.2,
                    x + radius * 0.6,
                    y - radius * 0.2,
                    1.6,
                    with_alpha(WHITE, 0.7),
                );
                draw_line(
                    x - radius * 0.15,
                    y - radius * 0.7,
                    x + radius * 0.15,
                    y + radius * 0.7,
                    1.2,
                    with_alpha(WHITE, 0.5),
                );
            }
        }
    }

    fn draw_power_aura(&self, position: Vec2, time: f32) {
        if let Some(power_kind) = self.power_kind {
            match power_kind {
                PowerKind::Feather => {
                    let pulse = (time * 6.0).sin() * 0.5 + 0.5;
                    let radius = BIRD_RADIUS + 6.0 + pulse * 2.0;
                    let alpha = 0.22 + 0.18 * pulse;
                    draw_circle_lines(
                        position.x,
                        position.y,
                        radius,
                        2.0,
                        with_alpha(COLOR_FEATHER, alpha),
                    );
                    draw_circle_lines(
                        position.x - 10.0,
                        position.y + 4.0,
                        radius * 0.6,
                        1.4,
                        with_alpha(COLOR_FEATHER, alpha * 0.7),
                    );
                }
                PowerKind::Shield => {
                    let pulse = (time * 5.0).sin() * 0.5 + 0.5;
                    let radius = BIRD_RADIUS + 8.0 + pulse * 1.5;
                    let alpha = 0.25 + 0.2 * pulse;
                    draw_circle_lines(
                        position.x,
                        position.y,
                        radius,
                        2.6,
                        with_alpha(COLOR_SHIELD, alpha),
                    );
                    draw_circle(
                        position.x,
                        position.y,
                        radius * 0.6,
                        with_alpha(COLOR_SHIELD, alpha * 0.15),
                    );
                }
            }
        }

        if self.boost_timer > 0.0 {
            let strength = (self.boost_timer / BOOST_DURATION).clamp(0.0, 1.0);
            let pulse = (time * 7.2).sin() * 0.5 + 0.5;
            let radius = BIRD_RADIUS + 10.0 + pulse * 3.0;
            let alpha = (0.18 + 0.22 * pulse) * strength;
            draw_circle_lines(
                position.x,
                position.y,
                radius,
                2.2,
                with_alpha(COLOR_BOOST, alpha),
            );
            draw_circle(
                position.x - 6.0,
                position.y + 3.0,
                radius * 0.45,
                with_alpha(COLOR_BOOST_CORE, alpha * 0.4),
            );
        }

        if self.shield_flash > 0.0 {
            let t = (self.shield_flash / 0.35).clamp(0.0, 1.0);
            let radius = BIRD_RADIUS + 12.0 + (1.0 - t) * 18.0;
            let alpha = 0.6 * t;
            draw_circle_lines(
                position.x,
                position.y,
                radius,
                2.6,
                with_alpha(COLOR_SHIELD_FLASH, alpha),
            );
        }

        if self.flow > 0 && !self.game_over {
            let flow_ratio = (self.flow as f32 / MAX_FLOW as f32).clamp(0.0, 1.0);
            let pulse = (time * 4.0).sin() * 0.5 + 0.5;
            let radius = BIRD_RADIUS + 3.0 + pulse * 1.8;
            let alpha = 0.08 + 0.14 * flow_ratio;
            draw_circle_lines(
                position.x,
                position.y,
                radius,
                1.0,
                with_alpha(COLOR_FLOW, alpha),
            );
        }
    }

    fn draw_ground(&self, ground_y: f32) {
        draw_rectangle(0.0, ground_y, SCREEN_W, GROUND_H, COLOR_GROUND);
        draw_rectangle(0.0, ground_y, SCREEN_W, 6.0, COLOR_GROUND_DARK);

        let stripe_w = 26.0;
        let stripe_h = 6.0;
        let mut x = -self.ground_scroll;
        while x < SCREEN_W + stripe_w {
            draw_rectangle(x, ground_y + 16.0, stripe_w, stripe_h, COLOR_GROUND_DARK);
            x += stripe_w + 18.0;
        }

        let mut x2 = -self.ground_scroll * 0.5;
        while x2 < SCREEN_W + 20.0 {
            draw_rectangle(x2, ground_y + 38.0, 12.0, 5.0, COLOR_GROUND_DARK);
            x2 += 40.0;
        }
    }

    fn spawn_clouds(&mut self) {
        self.clouds.clear();
        let max_y = (SCREEN_H - GROUND_H - 140.0).max(80.0);
        for _ in 0..CLOUD_COUNT {
            let scale = rand::gen_range(0.6, 1.35);
            let cloud = Cloud {
                x: rand::gen_range(0.0, SCREEN_W),
                y: rand::gen_range(40.0, max_y),
                speed: rand::gen_range(18.0, 42.0) * (0.7 + scale * 0.3),
                scale,
            };
            self.clouds.push(cloud);
        }
    }

    fn reset(&mut self) {
        self.bird_y = SCREEN_H / 2.0;
        self.bird_vy = 0.0;
        self.pipes.clear();
        self.collectibles.clear();
        self.score = 0;
        self.difficulty = 0.0;
        self.combo_count = 0;
        self.combo_timer = 0.0;
        self.flow = 0;
        self.flow_flash = 0.0;
        self.power_kind = None;
        self.power_timer = 0.0;
        self.boost_timer = 0.0;
        self.close_timer = 0.0;
        self.shield_flash = 0.0;
        self.ground_scroll = 0.0;
        self.game_over = false;
        self.started = false;
        self.paused = false;
        self.plane_pitch = 0.0;
        self.spawn_clouds();

        let mut x = SCREEN_W + 80.0;
        for _ in 0..PIPE_COUNT {
            self.spawn_pipe(x);
            x += self.current_pipe_spacing();
        }
    }

    fn ensure_pipes(&mut self) {
        if self.pipes.len() >= PIPE_COUNT {
            return;
        }

        let mut max_x = SCREEN_W;
        for pipe in &self.pipes {
            if pipe.x > max_x {
                max_x = pipe.x;
            }
        }

        while self.pipes.len() < PIPE_COUNT {
            max_x += self.current_pipe_spacing();
            self.spawn_pipe(max_x);
        }
    }

    fn spawn_pipe(&mut self, x: f32) {
        let pipe = self.create_pipe_at(x);
        let gap_y = pipe.gap_y;
        let gap = pipe.gap;
        self.pipes.push(pipe);

        if let Some(collectible) = Self::maybe_spawn_collectible(x + PIPE_WIDTH * 0.5, gap_y, gap) {
            self.collectibles.push(collectible);
        }
    }

    fn create_pipe_at(&self, x: f32) -> Pipe {
        let gap = self.current_pipe_gap();
        let ground_y = SCREEN_H - GROUND_H;
        let gap_half = gap / 2.0;
        let mut min_y = PIPE_MARGIN + gap_half;
        let mut max_y = ground_y - PIPE_MARGIN - gap_half;

        if let Some(previous_gap_y) = self.rightmost_gap_y() {
            let max_delta = self.max_gap_delta();
            min_y = min_y.max(previous_gap_y - max_delta);
            max_y = max_y.min(previous_gap_y + max_delta);
        }

        let gap_y = if max_y > min_y {
            rand::gen_range(min_y, max_y)
        } else {
            (min_y + max_y) / 2.0
        };

        Pipe {
            x,
            gap_y,
            gap,
            scored: false,
        }
    }

    fn maybe_spawn_collectible(x: f32, gap_y: f32, gap: f32) -> Option<Collectible> {
        let roll = rand::gen_range(0.0, 1.0);
        let kind = if roll < 0.50 {
            CollectibleKind::Star
        } else if roll < 0.66 {
            CollectibleKind::Feather
        } else if roll < 0.74 {
            CollectibleKind::Shield
        } else if roll < 0.82 {
            CollectibleKind::Boost
        } else {
            return None;
        };

        let offset = rand::gen_range(-gap * 0.25, gap * 0.25);
        let y_min = PIPE_MARGIN + 24.0;
        let y_max = SCREEN_H - GROUND_H - 24.0;
        let y = (gap_y + offset).clamp(y_min, y_max);
        let radius = match kind {
            CollectibleKind::Star => STAR_RADIUS,
            CollectibleKind::Feather => FEATHER_RADIUS,
            CollectibleKind::Shield => SHIELD_RADIUS,
            CollectibleKind::Boost => BOOST_RADIUS,
        };
        let bob_offset = rand::gen_range(0.0, std::f32::consts::TAU);

        Some(Collectible {
            x,
            y,
            kind,
            radius,
            bob_offset,
            collected: false,
        })
    }
}

fn rotate_vec(point: Vec2, angle: f32) -> Vec2 {
    let (sin, cos) = angle.sin_cos();
    vec2(point.x * cos - point.y * sin, point.x * sin + point.y * cos)
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

fn lerp_color(a: Color, b: Color, t: f32) -> Color {
    Color::new(
        lerp(a.r, b.r, t),
        lerp(a.g, b.g, t),
        lerp(a.b, b.b, t),
        lerp(a.a, b.a, t),
    )
}

fn with_alpha(color: Color, alpha: f32) -> Color {
    Color::new(color.r, color.g, color.b, alpha.clamp(0.0, 1.0))
}

fn sky_cycle(time: f32) -> f32 {
    (time * 0.04).sin() * 0.5 + 0.5
}

fn sky_palette(time: f32, flow: u32) -> (Color, Color, f32, f32) {
    let cycle = sky_cycle(time);
    let dusk = (1.0 - (cycle - 0.5).abs() * 2.0).clamp(0.0, 1.0);

    let mut top = lerp_color(COLOR_SKY_NIGHT_TOP, COLOR_SKY_DAY_TOP, cycle);
    let mut bottom = lerp_color(COLOR_SKY_NIGHT_BOTTOM, COLOR_SKY_DAY_BOTTOM, cycle);

    let dusk_weight = dusk * 0.6;
    top = lerp_color(top, COLOR_SKY_DUSK, dusk_weight);
    bottom = lerp_color(bottom, COLOR_SKY_DUSK, dusk_weight * 0.65);

    let flow_boost = (flow as f32 / MAX_FLOW as f32) * 0.14;
    top = lerp_color(top, WHITE, flow_boost);
    bottom = lerp_color(bottom, WHITE, flow_boost * 0.8);

    let night_strength = 1.0 - cycle;
    (top, bottom, night_strength, cycle)
}

fn pseudo_rand(seed: f32) -> f32 {
    (seed.sin() * 43758.5453).abs().fract()
}
