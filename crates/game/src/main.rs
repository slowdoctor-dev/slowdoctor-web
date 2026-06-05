//! "The Burnout Runner" — an endless-runner mini-game (Rust → WASM, Canvas 2D).
//!
//! An MD-Dev dodges work stressors: jump (Space / ↑) over ground obstacles
//! (textbooks, 404 bugs, server crashes), duck (↓) under flying ones (pagers,
//! Slack alerts), and grab espresso/vitamins for temporary burnout immunity.
//!
//! Physics are delta-time based (px/sec, px/sec²) so behaviour is identical at
//! 60/120/144 Hz. Mounts into `#game-canvas`; no-op if that element is absent.

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, KeyboardEvent};

// --- Logical canvas + world geometry (CSS scales the canvas; physics use these) ---
const W: f64 = 900.0;
const H: f64 = 260.0;
const BASELINE: f64 = 214.0; // ground line (player foot rests here)
const FLY_Y: f64 = 150.0; // top of flying obstacles

// --- Player ---
const PLAYER_X: f64 = 96.0;
const STAND_W: f64 = 34.0;
const STAND_H: f64 = 48.0;
const DUCK_W: f64 = 46.0;
const DUCK_H: f64 = 26.0;

// --- Physics (delta-time units) ---
const GRAVITY: f64 = 2600.0; // px/s²
const JUMP_V: f64 = -820.0; // px/s (negative = up)
const BASE_SPEED: f64 = 340.0; // px/s
const SPEED_RAMP: f64 = 1.6; // px/s gained per point of score
const MAX_SPEED: f64 = 760.0;
const SCORE_RATE: f64 = 9.0; // points ("patients cured") per second
const INVINCIBLE_SEC: f64 = 4.5;
const MAX_DT: f64 = 0.05; // clamp to avoid tunneling after tab switch

// --- Palette (matches the site theme) ---
const BG: &str = "#0a0a0a";
const FG: &str = "#ededed";
const MUTED: &str = "#888888";
const ACCENT: &str = "#b59768";
const BORDER: &str = "rgba(255,255,255,0.12)";
const CARD: &str = "rgba(255,255,255,0.05)";
const DANGER: &str = "#ff7a6b";
const TEAL: &str = "#7ed7c1";
const SKIN: &str = "#d8b48a";
const LAPTOP: &str = "#2b2b2b";
const SCREEN: &str = "#9ecbff";

#[derive(Clone, Copy, PartialEq)]
enum State {
    Ready,
    Playing,
    Over,
}

#[derive(Clone, Copy)]
enum ObKind {
    Books,
    Bug,
    Crash,
    Pager,
    Slack,
}

struct Obstacle {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
    kind: ObKind,
}

#[derive(Clone, Copy)]
enum PuKind {
    Espresso,
    Vitamin,
}

struct PowerUp {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
    kind: PuKind,
}

/// Minimal WebAudio blip generator (placeholders: keyboard click, milestone beep, flatline).
struct Audio {
    ctx: Option<web_sys::AudioContext>,
    muted: bool,
}

impl Audio {
    fn new() -> Self {
        Audio { ctx: None, muted: false }
    }

    fn ensure(&mut self) {
        if self.ctx.is_none() {
            self.ctx = web_sys::AudioContext::new().ok();
        }
        // Resume in case the context starts suspended (autoplay policy).
        if let Some(ctx) = &self.ctx {
            let _ = ctx.resume();
        }
    }

    fn blip(&self, freq: f64, dur: f64, kind: web_sys::OscillatorType, vol: f64) {
        if self.muted {
            return;
        }
        let Some(ctx) = &self.ctx else { return };
        let _ = (|| -> Result<(), JsValue> {
            let osc = ctx.create_oscillator()?;
            let gain = ctx.create_gain()?;
            osc.set_type(kind);
            osc.frequency().set_value(freq as f32);
            let t = ctx.current_time();
            gain.gain().set_value_at_time(vol as f32, t)?;
            gain.gain().exponential_ramp_to_value_at_time(0.0001, t + dur)?;
            osc.connect_with_audio_node(&gain)?;
            gain.connect_with_audio_node(&ctx.destination())?;
            osc.start_with_when(t)?;
            osc.stop_with_when(t + dur)?;
            Ok(())
        })();
    }

    fn click(&self) {
        self.blip(620.0, 0.05, web_sys::OscillatorType::Square, 0.05);
    }
    fn beep(&self) {
        self.blip(988.0, 0.09, web_sys::OscillatorType::Sine, 0.06);
    }
    fn flatline(&self) {
        self.blip(200.0, 0.7, web_sys::OscillatorType::Sine, 0.08);
    }
}

struct Game {
    ctx: CanvasRenderingContext2d,
    state: State,
    // player
    py: f64, // foot offset above baseline (<= 0 when airborne)
    vy: f64,
    on_ground: bool,
    duck_held: bool,
    touch_duck: bool,
    // world
    speed: f64,
    dist_to_next: f64,
    obstacles: Vec<Obstacle>,
    powerups: Vec<PowerUp>,
    next_powerup: f64,
    invincible: f64,
    score: f64,
    best: f64,
    last_milestone: i64,
    run_phase: f64,
    last_ts: Option<f64>,
    rng: u64,
    audio: Audio,
}

fn rand_unit(rng: &mut u64) -> f64 {
    // xorshift64* — deterministic per session, no JS calls in the hot loop.
    let mut x = *rng;
    x ^= x >> 12;
    x ^= x << 25;
    x ^= x >> 27;
    *rng = x;
    ((x.wrapping_mul(0x2545F4914F6CDD1D) >> 11) as f64) / ((1u64 << 53) as f64)
}

fn aabb(ax: f64, ay: f64, aw: f64, ah: f64, bx: f64, by: f64, bw: f64, bh: f64) -> bool {
    ax < bx + bw && ax + aw > bx && ay < by + bh && ay + ah > by
}

impl Game {
    fn new(ctx: CanvasRenderingContext2d) -> Self {
        let seed = (js_sys::Math::random() * (u64::MAX as f64)) as u64 | 1;
        Game {
            ctx,
            state: State::Ready,
            py: 0.0,
            vy: 0.0,
            on_ground: true,
            duck_held: false,
            touch_duck: false,
            speed: BASE_SPEED,
            dist_to_next: 500.0,
            obstacles: Vec::new(),
            powerups: Vec::new(),
            next_powerup: 9.0,
            invincible: 0.0,
            score: 0.0,
            best: 0.0,
            last_milestone: 0,
            run_phase: 0.0,
            last_ts: None,
            rng: seed,
            audio: Audio::new(),
        }
    }

    fn reset(&mut self) {
        self.state = State::Playing;
        self.py = 0.0;
        self.vy = 0.0;
        self.on_ground = true;
        self.duck_held = false;
        self.touch_duck = false;
        self.speed = BASE_SPEED;
        self.dist_to_next = 480.0;
        self.obstacles.clear();
        self.powerups.clear();
        self.next_powerup = 9.0;
        self.invincible = 0.0;
        self.score = 0.0;
        self.last_milestone = 0;
        self.run_phase = 0.0;
    }

    /// A jump/tap: starts the game, jumps, or restarts depending on state.
    fn primary_action(&mut self) {
        self.audio.ensure();
        match self.state {
            State::Ready | State::Over => self.reset(),
            State::Playing => {
                if self.on_ground {
                    self.vy = JUMP_V;
                    self.on_ground = false;
                    self.audio.click();
                }
            }
        }
    }

    fn set_duck(&mut self, held: bool) {
        self.duck_held = held;
    }

    fn ducking(&self) -> bool {
        self.duck_held && self.on_ground && self.state == State::Playing
    }

    fn player_box(&self) -> (f64, f64, f64, f64) {
        let (w, h) = if self.ducking() {
            (DUCK_W, DUCK_H)
        } else {
            (STAND_W, STAND_H)
        };
        let foot = BASELINE + self.py;
        (PLAYER_X, foot - h, w, h)
    }

    fn spawn_obstacle(&mut self) {
        let r = rand_unit(&mut self.rng);
        let kind = if r < 0.30 {
            if rand_unit(&mut self.rng) < 0.5 {
                ObKind::Pager
            } else {
                ObKind::Slack
            }
        } else if r < 0.55 {
            ObKind::Books
        } else if r < 0.80 {
            ObKind::Bug
        } else {
            ObKind::Crash
        };
        let (w, h, y) = match kind {
            ObKind::Books => (40.0, 44.0, BASELINE - 44.0),
            ObKind::Bug => (32.0, 32.0, BASELINE - 32.0),
            ObKind::Crash => (46.0, 40.0, BASELINE - 40.0),
            ObKind::Pager => (48.0, 30.0, FLY_Y),
            ObKind::Slack => (44.0, 32.0, FLY_Y - 4.0),
        };
        self.obstacles.push(Obstacle { x: W + 20.0, y, w, h, kind });
        // Gap in pixels (constant) → time gap shrinks as speed rises = harder.
        self.dist_to_next = 300.0 + rand_unit(&mut self.rng) * 320.0;
    }

    fn spawn_powerup(&mut self) {
        let kind = if rand_unit(&mut self.rng) < 0.5 {
            PuKind::Espresso
        } else {
            PuKind::Vitamin
        };
        // Float it so it takes a jump to grab (a real choice vs. dodging).
        self.powerups.push(PowerUp {
            x: W + 20.0,
            y: BASELINE - 96.0,
            w: 26.0,
            h: 30.0,
            kind,
        });
        self.next_powerup = 10.0 + rand_unit(&mut self.rng) * 9.0;
    }

    fn update(&mut self, dt: f64) {
        if self.state != State::Playing {
            self.run_phase += dt * 6.0; // idle leg shuffle on menus
            return;
        }

        self.speed = (BASE_SPEED + self.score * SPEED_RAMP).min(MAX_SPEED);
        self.run_phase += dt * self.speed * 0.04;

        // Vertical physics
        if !self.on_ground {
            self.vy += GRAVITY * dt;
            self.py += self.vy * dt;
            if self.py >= 0.0 {
                self.py = 0.0;
                self.vy = 0.0;
                self.on_ground = true;
            }
        }

        // Score + milestone beep
        self.score += dt * SCORE_RATE;
        let m = (self.score as i64) / 100;
        if m > self.last_milestone {
            self.last_milestone = m;
            self.audio.beep();
        }

        if self.invincible > 0.0 {
            self.invincible = (self.invincible - dt).max(0.0);
        }

        // Spawn + move obstacles
        self.dist_to_next -= self.speed * dt;
        if self.dist_to_next <= 0.0 {
            self.spawn_obstacle();
        }
        for o in &mut self.obstacles {
            o.x -= self.speed * dt;
        }
        self.obstacles.retain(|o| o.x + o.w > -10.0);

        // Spawn + move powerups
        self.next_powerup -= dt;
        if self.next_powerup <= 0.0 {
            self.spawn_powerup();
        }
        for p in &mut self.powerups {
            p.x -= self.speed * dt;
        }
        self.powerups.retain(|p| p.x + p.w > -10.0);

        // Collection BEFORE collision: an overlapping power-up grants immunity
        // this same frame, so it can't be "stolen" by a coincident obstacle.
        let (px, py, pw, ph) = self.player_box();
        let mut collected = false;
        self.powerups.retain(|p| {
            if aabb(px, py, pw, ph, p.x, p.y, p.w, p.h) {
                collected = true;
                false
            } else {
                true
            }
        });
        if collected {
            self.invincible = INVINCIBLE_SEC;
            self.audio.beep();
        }

        if self.invincible <= 0.0 {
            let hit = self
                .obstacles
                .iter()
                .any(|o| aabb(px, py, pw, ph, o.x + 3.0, o.y + 3.0, o.w - 6.0, o.h - 6.0));
            if hit {
                self.state = State::Over;
                self.best = self.best.max(self.score);
                self.audio.flatline();
                return;
            }
        }
    }

    // ---- Rendering helpers ----
    fn fill(&self, c: &str) {
        self.ctx.set_fill_style_str(c);
    }
    fn rect(&self, x: f64, y: f64, w: f64, h: f64) {
        self.ctx.fill_rect(x, y, w, h);
    }
    fn text(&self, s: &str, x: f64, y: f64, c: &str, font: &str, align: &str) {
        self.fill(c);
        self.ctx.set_font(font);
        self.ctx.set_text_align(align);
        let _ = self.ctx.fill_text(s, x, y);
    }

    fn render(&self) {
        // Background
        self.fill(BG);
        self.rect(0.0, 0.0, W, H);

        // Ground line + scrolling ticks for motion feedback
        self.fill(BORDER);
        self.rect(0.0, BASELINE, W, 2.0);
        self.fill(MUTED);
        let off = (self.run_phase * 6.0) % 44.0;
        let mut x = -off;
        while x < W {
            self.rect(x, BASELINE + 8.0, 16.0, 2.0);
            x += 44.0;
        }

        for p in &self.powerups {
            self.draw_powerup(p);
        }
        for o in &self.obstacles {
            self.draw_obstacle(o);
        }
        self.draw_player();
        self.draw_hud();

        match self.state {
            State::Ready => self.draw_center_panel(
                "The Burnout Runner",
                "Space / Tap to start    ·    \u{2191} jump    ·    \u{2193} duck",
                None,
            ),
            State::Over => self.draw_center_panel(
                "SYSTEM CRASH & BURNOUT!",
                &format!("Patients cured: {}    ·    Best: {}", self.score as i64, self.best.max(self.score) as i64),
                Some("[ Restart ]  (Space / Tap)"),
            ),
            State::Playing => {}
        }
    }

    fn draw_player(&self) {
        let (x, top, w, h) = self.player_box();
        // Invincibility: glow + flicker
        let flicker = self.invincible > 0.0 && ((self.invincible * 14.0) as i64) % 2 == 0;
        if self.invincible > 0.0 {
            self.fill("rgba(181,151,104,0.25)");
            self.rect(x - 6.0, top - 6.0, w + 12.0, h + 12.0);
        }
        if flicker {
            return;
        }

        if self.ducking() {
            // Crouched: head left, coat body, tucked legs
            self.fill(SKIN);
            self.rect(x, top, 14.0, 12.0);
            self.fill(FG); // coat
            self.rect(x + 8.0, top + 6.0, w - 8.0, h - 12.0);
            self.fill(ACCENT); // stethoscope dash
            self.rect(x + 10.0, top + 9.0, 10.0, 2.0);
            self.fill("#1a1a1a"); // legs
            self.rect(x + 10.0, top + h - 6.0, 10.0, 6.0);
            self.rect(x + 26.0, top + h - 6.0, 10.0, 6.0);
            return;
        }

        // Head
        self.fill(SKIN);
        self.rect(x + 9.0, top, 16.0, 13.0);
        self.fill("#1a1a1a"); // hair
        self.rect(x + 9.0, top, 16.0, 4.0);
        // Coat
        self.fill(FG);
        self.rect(x, top + 13.0, w, h - 24.0);
        // Stethoscope (accent): collar + disc
        self.fill(ACCENT);
        self.rect(x + 6.0, top + 14.0, 3.0, 12.0);
        self.rect(x + w - 9.0, top + 14.0, 3.0, 12.0);
        self.ctx.begin_path();
        let _ = self.ctx.arc(x + w / 2.0, top + 28.0, 3.0, 0.0, std::f64::consts::PI * 2.0);
        self.fill(TEAL);
        self.ctx.fill();
        // Laptop held in front
        self.fill(LAPTOP);
        self.rect(x + w - 6.0, top + 18.0, 12.0, 9.0);
        self.fill(SCREEN);
        self.rect(x + w - 4.0, top + 20.0, 8.0, 5.0);
        // Legs (run cycle) or tucked when airborne
        self.fill("#1a1a1a");
        if self.on_ground {
            let s = (self.run_phase).sin();
            let l1 = if s > 0.0 { 0.0 } else { 4.0 };
            let l2 = if s > 0.0 { 4.0 } else { 0.0 };
            self.rect(x + 6.0, top + h - 11.0, 8.0, 11.0 - l1);
            self.rect(x + w - 14.0, top + h - 11.0, 8.0, 11.0 - l2);
        } else {
            self.rect(x + 6.0, top + h - 8.0, 8.0, 8.0);
            self.rect(x + w - 14.0, top + h - 8.0, 8.0, 8.0);
        }
    }

    fn draw_obstacle(&self, o: &Obstacle) {
        match o.kind {
            ObKind::Books => {
                let colors = [ACCENT, TEAL, DANGER];
                let n = 3;
                let bh = o.h / n as f64;
                for i in 0..n {
                    self.fill(colors[i % colors.len()]);
                    let inset = (i as f64) * 2.0;
                    self.rect(o.x + inset, o.y + bh * i as f64, o.w - inset * 2.0, bh - 2.0);
                    self.fill("rgba(0,0,0,0.35)"); // spine
                    self.rect(o.x + inset + 3.0, o.y + bh * i as f64 + 2.0, 2.0, bh - 6.0);
                }
            }
            ObKind::Bug => {
                self.fill(DANGER);
                self.rect(o.x, o.y, o.w, o.h);
                self.text("404", o.x + o.w / 2.0, o.y + o.h / 2.0 + 4.0, "#1a1a1a", "bold 12px ui-monospace, monospace", "center");
            }
            ObKind::Crash => {
                self.fill("#1a1a1a");
                self.rect(o.x, o.y, o.w, o.h);
                self.fill(DANGER);
                self.rect(o.x, o.y, o.w, 4.0);
                self.text("\u{26A0}", o.x + o.w / 2.0, o.y + o.h / 2.0 + 6.0, DANGER, "bold 18px sans-serif", "center");
                self.text("500", o.x + o.w / 2.0, o.y + o.h - 5.0, MUTED, "bold 9px ui-monospace, monospace", "center");
            }
            ObKind::Pager => {
                self.fill(CARD);
                self.rect(o.x, o.y, o.w, o.h);
                self.fill(BORDER);
                self.rect(o.x, o.y, o.w, 2.0);
                self.text("BEEP", o.x + o.w / 2.0, o.y + o.h / 2.0 + 4.0, TEAL, "bold 11px ui-monospace, monospace", "center");
            }
            ObKind::Slack => {
                self.fill("#1a1a1a");
                self.rect(o.x, o.y, o.w, o.h);
                self.fill(ACCENT);
                self.rect(o.x, o.y, 4.0, o.h);
                self.text("@here", o.x + o.w / 2.0 + 2.0, o.y + o.h / 2.0 + 4.0, FG, "bold 10px ui-monospace, monospace", "center");
            }
        }
    }

    fn draw_powerup(&self, p: &PowerUp) {
        match p.kind {
            PuKind::Espresso => {
                self.fill("#3a2a1a"); // cup
                self.rect(p.x, p.y + 8.0, p.w, p.h - 8.0);
                self.fill("#5a3a22");
                self.rect(p.x + 2.0, p.y + 10.0, p.w - 4.0, 6.0);
                self.fill(ACCENT); // steam
                self.rect(p.x + 6.0, p.y, 3.0, 7.0);
                self.rect(p.x + 14.0, p.y + 1.0, 3.0, 6.0);
            }
            PuKind::Vitamin => {
                self.fill(TEAL);
                self.rect(p.x, p.y + 6.0, p.w, (p.h - 6.0) / 2.0);
                self.fill(FG);
                self.rect(p.x, p.y + 6.0 + (p.h - 6.0) / 2.0, p.w, (p.h - 6.0) / 2.0);
                self.fill("rgba(0,0,0,0.25)");
                self.rect(p.x + p.w / 2.0 - 1.0, p.y + 6.0, 2.0, p.h - 6.0);
            }
        }
    }

    fn draw_hud(&self) {
        let cured = self.score as i64;
        let salary = cured * 7;
        self.text(&format!("Patients Cured  {cured}"), W - 16.0, 28.0, FG, "bold 16px ui-monospace, monospace", "right");
        self.text(&format!("$ {salary}", ), W - 16.0, 46.0, MUTED, "12px ui-monospace, monospace", "right");
        if self.best > 0.0 {
            self.text(&format!("Best  {}", self.best as i64), 16.0, 28.0, MUTED, "12px ui-monospace, monospace", "left");
        }
        if self.invincible > 0.0 {
            self.text(
                &format!("\u{2615} BURNOUT RELIEF {:.1}s", self.invincible),
                16.0,
                46.0,
                ACCENT,
                "bold 12px ui-monospace, monospace",
                "left",
            );
        }
    }

    fn draw_center_panel(&self, title: &str, sub: &str, action: Option<&str>) {
        self.fill("rgba(10,10,10,0.72)");
        self.rect(0.0, 0.0, W, H);
        self.text(title, W / 2.0, H / 2.0 - 14.0, FG, "bold 26px sans-serif", "center");
        self.text(sub, W / 2.0, H / 2.0 + 14.0, MUTED, "14px ui-monospace, monospace", "center");
        if let Some(a) = action {
            self.text(a, W / 2.0, H / 2.0 + 44.0, ACCENT, "bold 15px ui-monospace, monospace", "center");
        }
    }
}

fn window() -> web_sys::Window {
    web_sys::window().expect("window")
}

fn request_animation_frame(f: &Closure<dyn FnMut(f64)>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("requestAnimationFrame");
}

fn main() {
    console_error_panic_hook::set_once();

    let document = window().document().expect("document");
    let Some(canvas) = document.get_element_by_id("game-canvas") else {
        return; // not on a page with the game
    };
    let canvas: HtmlCanvasElement = match canvas.dyn_into() {
        Ok(c) => c,
        Err(_) => return,
    };
    canvas.set_width(W as u32);
    canvas.set_height(H as u32);
    let _ = canvas.set_attribute("tabindex", "0");

    let ctx: CanvasRenderingContext2d = canvas
        .get_context("2d")
        .ok()
        .flatten()
        .expect("2d context")
        .dyn_into()
        .expect("ctx");

    let game = Rc::new(RefCell::new(Game::new(ctx)));

    // Keyboard (listens on the canvas; click-to-focus avoids hijacking page scroll).
    {
        let game = game.clone();
        let on_keydown = Closure::<dyn FnMut(KeyboardEvent)>::new(move |e: KeyboardEvent| {
            match e.code().as_str() {
                "Space" | "ArrowUp" | "KeyW" => {
                    e.prevent_default();
                    if !e.repeat() {
                        game.borrow_mut().primary_action();
                    }
                }
                "ArrowDown" | "KeyS" => {
                    e.prevent_default();
                    game.borrow_mut().set_duck(true);
                }
                "Enter" => {
                    let mut g = game.borrow_mut();
                    if g.state != State::Playing {
                        g.primary_action();
                    }
                }
                "KeyM" => {
                    if !e.repeat() {
                        let mut g = game.borrow_mut();
                        g.audio.muted = !g.audio.muted;
                    }
                }
                _ => {}
            }
        });
        canvas
            .add_event_listener_with_callback("keydown", on_keydown.as_ref().unchecked_ref())
            .expect("keydown");
        on_keydown.forget();
    }
    {
        let game = game.clone();
        let on_keyup = Closure::<dyn FnMut(KeyboardEvent)>::new(move |e: KeyboardEvent| {
            if matches!(e.code().as_str(), "ArrowDown" | "KeyS") {
                game.borrow_mut().set_duck(false);
            }
        });
        canvas
            .add_event_listener_with_callback("keyup", on_keyup.as_ref().unchecked_ref())
            .expect("keyup");
        on_keyup.forget();
    }
    // Pointer down: tap upper area = jump/start/restart; press-and-hold the
    // lower area = duck (gives mobile a duck control).
    {
        let game = game.clone();
        let cv = canvas.clone();
        let on_down = Closure::<dyn FnMut(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
            e.prevent_default();
            let _ = cv.focus();
            let rect = cv.get_bounding_client_rect();
            let ly = ((e.client_y() as f64) - rect.top()) / rect.height().max(1.0) * H;
            let mut g = game.borrow_mut();
            if g.state == State::Playing && ly > H * 0.55 {
                g.set_duck(true);
                g.touch_duck = true;
            } else {
                g.primary_action();
            }
        });
        canvas
            .add_event_listener_with_callback("pointerdown", on_down.as_ref().unchecked_ref())
            .expect("pointerdown");
        on_down.forget();
    }
    // Pointer release: end a touch-duck.
    {
        let game = game.clone();
        let on_up = Closure::<dyn FnMut(web_sys::MouseEvent)>::new(move |_e: web_sys::MouseEvent| {
            let mut g = game.borrow_mut();
            if g.touch_duck {
                g.set_duck(false);
                g.touch_duck = false;
            }
        });
        for ev in ["pointerup", "pointercancel", "pointerleave"] {
            canvas
                .add_event_listener_with_callback(ev, on_up.as_ref().unchecked_ref())
                .expect("pointerup");
        }
        on_up.forget();
    }
    // Losing window focus mid-hold must not leave duck stuck on.
    {
        let game = game.clone();
        let on_blur = Closure::<dyn FnMut(JsValue)>::new(move |_v: JsValue| {
            let mut g = game.borrow_mut();
            g.set_duck(false);
            g.touch_duck = false;
        });
        window()
            .add_event_listener_with_callback("blur", on_blur.as_ref().unchecked_ref())
            .expect("blur");
        on_blur.forget();
    }

    // Main loop
    let f = Rc::new(RefCell::new(None::<Closure<dyn FnMut(f64)>>));
    let g = f.clone();
    *g.borrow_mut() = Some(Closure::new(move |ts: f64| {
        {
            let mut game = game.borrow_mut();
            let dt = match game.last_ts {
                Some(prev) => ((ts - prev) / 1000.0).clamp(0.0, MAX_DT),
                None => 0.0,
            };
            game.last_ts = Some(ts);
            game.update(dt);
            game.render();
        }
        request_animation_frame(f.borrow().as_ref().unwrap());
    }));
    request_animation_frame(g.borrow().as_ref().unwrap());
}
