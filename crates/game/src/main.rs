//! A minimal endless-runner mini-game (Rust → WASM, Canvas 2D).
//!
//! Jump (Space / ↑ / tap) over ground obstacles. Survive to raise the score.
//! Delta-time physics (px/sec, px/sec²) keep behaviour identical at 60/120/144 Hz.
//! Mounts into `#game-canvas`; no-op if that element is absent.

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, KeyboardEvent};

// --- Logical canvas + world geometry (CSS scales the canvas; physics use these) ---
const W: f64 = 900.0;
const H: f64 = 260.0;
const BASELINE: f64 = 214.0; // ground line (player foot rests here)

// --- Player ---
const PLAYER_X: f64 = 96.0;
const PW: f64 = 34.0;
const PH: f64 = 46.0;

// --- Physics (delta-time units) ---
const GRAVITY: f64 = 2600.0; // px/s²
const JUMP_V: f64 = -820.0; // px/s (negative = up)
const BASE_SPEED: f64 = 340.0; // px/s
const SPEED_RAMP: f64 = 1.6; // px/s gained per point of score
const MAX_SPEED: f64 = 760.0;
const SCORE_RATE: f64 = 9.0; // score points per second survived
const MAX_DT: f64 = 0.05; // clamp to avoid tunneling after tab switch

// --- Palette (matches the site theme) ---
const BG: &str = "#0a0a0a";
const FG: &str = "#ededed";
const MUTED: &str = "#888888";
const ACCENT: &str = "#b59768";
const BORDER: &str = "rgba(255,255,255,0.12)";

#[derive(Clone, Copy, PartialEq)]
enum State {
    Ready,
    Playing,
    Over,
}

struct Obstacle {
    x: f64,
    w: f64,
    h: f64,
}

/// Minimal WebAudio blip generator (jump click, milestone beep, game-over tone).
struct Audio {
    ctx: Option<web_sys::AudioContext>,
    muted: bool,
}

impl Audio {
    fn new() -> Self {
        Audio {
            ctx: None,
            muted: false,
        }
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
            gain.gain()
                .exponential_ramp_to_value_at_time(0.0001, t + dur)?;
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
    fn over(&self) {
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
    // world
    speed: f64,
    dist_to_next: f64,
    obstacles: Vec<Obstacle>,
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
            speed: BASE_SPEED,
            dist_to_next: 500.0,
            obstacles: Vec::new(),
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
        self.speed = BASE_SPEED;
        self.dist_to_next = 480.0;
        self.obstacles.clear();
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

    fn player_box(&self) -> (f64, f64, f64, f64) {
        let foot = BASELINE + self.py;
        (PLAYER_X, foot - PH, PW, PH)
    }

    fn spawn_obstacle(&mut self) {
        let w = 18.0 + rand_unit(&mut self.rng) * 16.0;
        let h = 28.0 + rand_unit(&mut self.rng) * 26.0;
        self.obstacles.push(Obstacle { x: W + 20.0, w, h });
        // Gap in pixels (constant) → time gap shrinks as speed rises = harder.
        self.dist_to_next = 300.0 + rand_unit(&mut self.rng) * 320.0;
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

        // Spawn + move obstacles
        self.dist_to_next -= self.speed * dt;
        if self.dist_to_next <= 0.0 {
            self.spawn_obstacle();
        }
        for o in &mut self.obstacles {
            o.x -= self.speed * dt;
        }
        self.obstacles.retain(|o| o.x + o.w > -10.0);

        // Collision
        let (px, py, pw, ph) = self.player_box();
        let hit = self.obstacles.iter().any(|o| {
            aabb(
                px,
                py,
                pw,
                ph,
                o.x + 3.0,
                BASELINE - o.h + 3.0,
                o.w - 6.0,
                o.h - 6.0,
            )
        });
        if hit {
            self.state = State::Over;
            self.best = self.best.max(self.score);
            self.audio.over();
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

        for o in &self.obstacles {
            self.draw_obstacle(o);
        }
        self.draw_player();
        self.draw_hud();

        match self.state {
            State::Ready => self.draw_center_panel("Press Space / Tap to start", None, None),
            State::Over => self.draw_center_panel(
                "Game Over",
                Some(&format!(
                    "Score {}    ·    Best {}",
                    self.score as i64,
                    self.best.max(self.score) as i64
                )),
                Some("Space / Tap to restart"),
            ),
            State::Playing => {}
        }
    }

    fn draw_player(&self) {
        let (x, top, w, h) = self.player_box();
        // Head
        self.fill(FG);
        self.rect(x + 9.0, top, 16.0, 13.0);
        // Body
        self.rect(x, top + 13.0, w, h - 24.0);
        // Legs: run cycle on the ground, tucked while airborne
        if self.on_ground {
            let s = self.run_phase.sin();
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
        let y = BASELINE - o.h;
        self.fill(ACCENT);
        self.rect(o.x, y, o.w, o.h);
    }

    fn draw_hud(&self) {
        self.text(
            &format!("Score  {}", self.score as i64),
            W - 16.0,
            28.0,
            FG,
            "bold 16px ui-monospace, monospace",
            "right",
        );
        if self.best > 0.0 {
            self.text(
                &format!("Best  {}", self.best as i64),
                16.0,
                28.0,
                MUTED,
                "12px ui-monospace, monospace",
                "left",
            );
        }
    }

    fn draw_center_panel(&self, title: &str, sub: Option<&str>, action: Option<&str>) {
        self.fill("rgba(10,10,10,0.72)");
        self.rect(0.0, 0.0, W, H);
        self.text(
            title,
            W / 2.0,
            H / 2.0 - 6.0,
            FG,
            "bold 24px sans-serif",
            "center",
        );
        if let Some(s) = sub {
            self.text(
                s,
                W / 2.0,
                H / 2.0 + 22.0,
                MUTED,
                "14px ui-monospace, monospace",
                "center",
            );
        }
        if let Some(a) = action {
            self.text(
                a,
                W / 2.0,
                H / 2.0 + 48.0,
                ACCENT,
                "bold 14px ui-monospace, monospace",
                "center",
            );
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
    // DOM contract: this ID is rendered by site::pages::home.
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
    // Pointer: focus + jump/start/restart (mobile-friendly tap).
    {
        let game = game.clone();
        let cv = canvas.clone();
        let on_down =
            Closure::<dyn FnMut(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
                e.prevent_default();
                let _ = cv.focus();
                game.borrow_mut().primary_action();
            });
        canvas
            .add_event_listener_with_callback("pointerdown", on_down.as_ref().unchecked_ref())
            .expect("pointerdown");
        on_down.forget();
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
