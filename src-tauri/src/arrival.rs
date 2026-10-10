//! How the window of the flyout style arrives and leaves.
//!
//! That style's card is its window, with the system's material behind it, so the page has
//! nothing to move: what travels and fades is the window itself. The other styles are drawn
//! inside a still, transparent window and play their arrival on the page.

use crate::overlay_settings::OverlayEntrance;
use parking_lot::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

/// Whether the window is on its way in or on its way out.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Way {
    In,
    Out,
}

/// One movement of the window, from wherever it stands.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Movement {
    entrance: OverlayEntrance,
    way: Way,
    length: Duration,
}

impl Movement {
    /// The movement the look asks for. Someone who asked for less movement gets a short fade.
    pub fn of(entrance: OverlayEntrance, way: Way, reduced: bool) -> Self {
        let entrance = if reduced { OverlayEntrance::Fade } else { entrance };
        let length = match (way, entrance, reduced) {
            (Way::Out, _, _) => LEAVE,
            (Way::In, _, true) => Duration::from_millis(200),
            (Way::In, OverlayEntrance::Fade, false) => Duration::from_millis(420),
            (Way::In, _, false) => Duration::from_millis(560),
        };
        Self { entrance, way, length }
    }

    /// How far from its place the window starts or ends, in logical pixels.
    fn reach(self) -> f64 {
        match (self.entrance, self.way) {
            (OverlayEntrance::Fade, _) => 0.0,
            (OverlayEntrance::Slide, Way::In) => 30.0,
            (OverlayEntrance::Slide, Way::Out) => 18.0,
            (OverlayEntrance::Bounce, Way::In) => 22.0,
            (OverlayEntrance::Bounce, Way::Out) => 10.0,
        }
    }
}

/// How long the window takes to leave. The page keeps its content for the same time, see
/// `LEAVE_MS` in `src/lib/overlay.ts`.
const LEAVE: Duration = Duration::from_millis(220);

/// Where the window stands in its arrival: how much of it shows, from nothing to all of it,
/// and how far it is from its place, in logical pixels towards the edge it came from.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Standing {
    pub shown: f64,
    pub away: f64,
}

impl Standing {
    pub const GONE: Self = Self { shown: 0.0, away: 0.0 };
    const THERE: Self = Self { shown: 1.0, away: 0.0 };

    /// Where the window stands once `elapsed` of the movement has passed, from 0 to 1, having
    /// started here. A movement that interrupts another therefore goes on from where the
    /// window is, and never jumps.
    pub fn after(self, movement: Movement, elapsed: f64) -> Self {
        let elapsed = elapsed.clamp(0.0, 1.0);
        match movement.way {
            Way::In => {
                // A window that shows nothing has not left its starting point yet.
                let from = if self.shown <= 0.0 { movement.reach() } else { self.away };
                let travelled = match movement.entrance {
                    OverlayEntrance::Bounce => past_and_back(elapsed),
                    _ => slowing(elapsed),
                };
                Self { shown: self.shown + (1.0 - self.shown) * slowing(elapsed), away: from * (1.0 - travelled) }
            }
            Way::Out => {
                let gathered = elapsed * elapsed;
                Self { shown: self.shown * (1.0 - gathered), away: self.away + (movement.reach() - self.away) * gathered }
            }
        }
    }
}

/// Fast at first and settling: the curve the page's arrivals follow.
fn slowing(elapsed: f64) -> f64 {
    1.0 - (1.0 - elapsed).powi(5)
}

/// The same, going a little past the end before coming back to it.
fn past_and_back(elapsed: f64) -> f64 {
    const PAST: f64 = 1.70158;
    let left = elapsed - 1.0;
    1.0 + (PAST + 1.0) * left.powi(3) + PAST * left.powi(2)
}

/// The side of the screen a window arrives from, and leaves towards.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Top,
    Bottom,
    Left,
    Right,
}

/// What a movement is played on: the window, the corner it rests at, the scale of its screen
/// and the side it comes from.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(not(windows), allow(dead_code))]
pub struct Stage {
    pub window: isize,
    pub rest: (i32, i32),
    pub scale: f64,
    pub from: Side,
}

impl Stage {
    /// The corner of the window when it stands that far from its place.
    #[cfg_attr(not(windows), allow(dead_code))]
    fn corner(&self, standing: Standing) -> (i32, i32) {
        let shift = (standing.away * self.scale).round() as i32;
        let (x, y) = self.rest;
        match self.from {
            Side::Top => (x, y - shift),
            Side::Bottom => (x, y + shift),
            Side::Left => (x - shift, y),
            Side::Right => (x + shift, y),
        }
    }
}

static STANDING: Mutex<Standing> = Mutex::new(Standing::GONE);

/// How many movements were asked for, which is how one learns that another took over.
static ASKED: AtomicU64 = AtomicU64::new(0);

/// Take the window for a movement, from whichever one has it. Taken where the movement is
/// asked for and not on the thread that plays it, so that two of them asked one after the
/// other are played in that order whichever thread starts first.
pub fn turn() -> u64 {
    ASKED.fetch_add(1, Ordering::SeqCst) + 1
}

/// Whether the window is whole and in its place, so that where it is now is where it rests.
pub fn arrived() -> bool {
    *STANDING.lock() == Standing::THERE
}

/// The window was hidden: its next arrival starts from nothing.
pub fn forget() {
    *STANDING.lock() = Standing::GONE;
}

/// Put the window where the movement starts, before it is shown.
pub fn prepare(stage: &Stage, movement: Movement) {
    draw(stage, STANDING.lock().after(movement, 0.0));
}

/// Play the movement to its end, a step a frame, on the calling thread, for as long as the
/// turn it was given is the last one taken. Answers false when another movement was asked
/// for meanwhile, and stops there: the window is that one's.
pub fn play(stage: &Stage, movement: Movement, turn: u64) -> bool {
    let from = *STANDING.lock();
    let started = Instant::now();
    loop {
        if ASKED.load(Ordering::SeqCst) != turn {
            return false;
        }
        let elapsed = (started.elapsed().as_secs_f64() / movement.length.as_secs_f64()).min(1.0);
        let standing = from.after(movement, elapsed);
        *STANDING.lock() = standing;
        draw(stage, standing);
        if elapsed >= 1.0 {
            return true;
        }
        next_frame();
    }
}

/// Move the window and set how much of it shows. A layered window fades whole, the system's
/// material and the page alike, and the material follows a move frame by frame.
#[cfg(windows)]
fn draw(stage: &Stage, standing: Standing) {
    use windows::Win32::Foundation::{COLORREF, HWND};
    use windows::Win32::UI::WindowsAndMessaging::{
        SetLayeredWindowAttributes, SetWindowPos, LWA_ALPHA, SWP_ASYNCWINDOWPOS, SWP_NOACTIVATE, SWP_NOSIZE,
        SWP_NOZORDER,
    };

    let window = HWND(stage.window as _);
    let (x, y) = stage.corner(standing);
    let alpha = (standing.shown.clamp(0.0, 1.0) * 255.0).round() as u8;
    // ASYNCWINDOWPOS, as in `overlay::raise`: this is not the thread that owns the window.
    unsafe {
        let _ = SetLayeredWindowAttributes(window, COLORREF(0), alpha, LWA_ALPHA);
        let _ = SetWindowPos(window, None, x, y, 0, 0, SWP_ASYNCWINDOWPOS | SWP_NOSIZE | SWP_NOZORDER | SWP_NOACTIVATE);
    }
}

#[cfg(not(windows))]
fn draw(_stage: &Stage, _standing: Standing) {}

/// Wait for the compositor's next frame, or about one when it will not say.
#[cfg(windows)]
fn next_frame() {
    if unsafe { windows::Win32::Graphics::Dwm::DwmFlush() }.is_err() {
        std::thread::sleep(Duration::from_millis(8));
    }
}

#[cfg(not(windows))]
fn next_frame() {
    std::thread::sleep(Duration::from_millis(8));
}

#[cfg(test)]
mod tests {
    use super::*;

    const ENTRANCES: [OverlayEntrance; 3] = [OverlayEntrance::Bounce, OverlayEntrance::Slide, OverlayEntrance::Fade];

    #[test]
    fn an_arrival_ends_in_place_and_whole() {
        for entrance in ENTRANCES {
            let arrived = Standing::GONE.after(Movement::of(entrance, Way::In, false), 1.0);
            assert!((arrived.shown - 1.0).abs() < 1e-9, "{entrance:?} shows {}", arrived.shown);
            assert!(arrived.away.abs() < 1e-9, "{entrance:?} rests {} away", arrived.away);
        }
    }

    #[test]
    fn an_arrival_starts_unseen_and_away() {
        let slide = Standing::GONE.after(Movement::of(OverlayEntrance::Slide, Way::In, false), 0.0);
        assert_eq!(slide, Standing { shown: 0.0, away: 30.0 });
        let fade = Standing::GONE.after(Movement::of(OverlayEntrance::Fade, Way::In, false), 0.0);
        assert_eq!(fade, Standing::GONE);
    }

    #[test]
    fn a_bounce_goes_past_its_place_and_a_slide_does_not() {
        let passes = |entrance| {
            let movement = Movement::of(entrance, Way::In, false);
            (1..100).any(|step| Standing::GONE.after(movement, f64::from(step) / 100.0).away < -0.5)
        };
        assert!(passes(OverlayEntrance::Bounce));
        assert!(!passes(OverlayEntrance::Slide));
    }

    #[test]
    fn a_departure_ends_unseen() {
        let there = Standing { shown: 1.0, away: 0.0 };
        for entrance in ENTRANCES {
            let left = there.after(Movement::of(entrance, Way::Out, false), 1.0);
            assert!(left.shown.abs() < 1e-9);
        }
        assert_eq!(there.after(Movement::of(OverlayEntrance::Slide, Way::Out, false), 1.0).away, 18.0);
    }

    #[test]
    fn a_movement_that_interrupts_another_starts_where_the_window_is() {
        let arriving = Standing::GONE.after(Movement::of(OverlayEntrance::Slide, Way::In, false), 0.3);
        assert_eq!(arriving.after(Movement::of(OverlayEntrance::Slide, Way::Out, false), 0.0), arriving);
        let leaving = Standing { shown: 1.0, away: 0.0 }.after(Movement::of(OverlayEntrance::Slide, Way::Out, false), 0.6);
        assert_eq!(leaving.after(Movement::of(OverlayEntrance::Slide, Way::In, false), 0.0), leaving);
    }

    #[test]
    fn less_movement_is_a_short_fade_whatever_was_picked() {
        let movement = Movement::of(OverlayEntrance::Bounce, Way::In, true);
        assert_eq!(movement.length, Duration::from_millis(200));
        assert_eq!(Standing::GONE.after(movement, 0.0).away, 0.0);
    }

    #[test]
    fn the_window_comes_from_the_side_it_sits_on() {
        let standing = Standing { shown: 0.0, away: 30.0 };
        let stage = Stage { window: 0, rest: (100, 500), scale: 1.5, from: Side::Bottom };
        assert_eq!(stage.corner(standing), (100, 545));
        assert_eq!(Stage { from: Side::Top, ..stage }.corner(standing), (100, 455));
        assert_eq!(Stage { from: Side::Left, ..stage }.corner(standing), (55, 500));
        assert_eq!(Stage { from: Side::Right, ..stage }.corner(standing), (145, 500));
    }
}
