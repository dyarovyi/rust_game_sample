use rusty_engine::prelude::*;
use rand::prelude::*;

struct GameState {
    high_score: u32,
    current_score: u32,
    enemy_labels: Vec<String>,
    ferris_index: u32,
    spawn_timer: Timer,
}

impl Default for GameState {
    fn default() -> Self {
        Self { 
            high_score: 0, 
            current_score: 0, 
            enemy_labels: Vec::new(), 
            ferris_index: 0,
            spawn_timer: Timer::from_seconds(2.0, true),
        }
    }
}

fn main() {
    let mut game = Game::new();
    
    game.window_settings(WindowDescriptor {
        title: "Game".to_string(),
        width: 600.0,
        height: 400.0,
        ..Default::default()
    });
    game.audio_manager.play_music(MusicPreset::WhimsicalPopsicle, 0.1);

    let current_score = game.add_text("current_score", "Current score: 0");
    current_score.translation = Vec2::new(520.0, 320.0);

    let high_score = game.add_text("high_score", "High score: 0");
    high_score.translation = Vec2::new(-520.0, 320.0);

    let player = game.add_sprite("player", SpritePreset::RacingCarBlue);
    player.translation = Vec2::new(0.0, 0.0);
    player.rotation = UP;
    player.scale = 1.0;
    player.collision = true;

    game.add_logic(game_logic);
    game.run(GameState::default());
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    const MOVEMENT_SPEED: f32 = 100.0;

    for event in engine.collision_events.drain(..) {
        if event.state == CollisionState::Begin && event.pair.one_starts_with("player") {
            for label in [event.pair.0, event.pair.1] {
                if label != "player" {
                    engine.sprites.remove(&label);
                }
            }
            game_state.current_score += 1;
            let current_score = engine.texts.get_mut("current_score").unwrap();
            current_score.value = format!("Current score: {}", game_state.current_score);

            if game_state.high_score < game_state.current_score {
                game_state.high_score = game_state.current_score;
                let high_score = engine.texts.get_mut("high_score").unwrap();
                high_score.value = format!("High score: {}", game_state.high_score);
            }

            engine.audio_manager.play_sfx(SfxPreset::Minimize1, 0.3);
        }
    }

    let player = engine.sprites.get_mut("player").unwrap();

    if engine.keyboard_state.pressed_any(&[KeyCode::W, KeyCode::Up]) {
        player.translation.y += MOVEMENT_SPEED * engine.delta_f32;
    }
    if engine.keyboard_state.pressed_any(&[KeyCode::S, KeyCode::Down]) {
        player.translation.y -= MOVEMENT_SPEED * engine.delta_f32;
    }
    if engine.keyboard_state.pressed_any(&[KeyCode::D, KeyCode::Right]) {
        player.translation.x += MOVEMENT_SPEED * engine.delta_f32;
    }
    if engine.keyboard_state.pressed_any(&[KeyCode::A, KeyCode::Left]) {
        player.translation.x -= MOVEMENT_SPEED * engine.delta_f32;
    }

    if game_state.spawn_timer.tick(engine.delta).just_finished() {
        let label = format!("ferris{}", game_state.ferris_index);
        game_state.enemy_labels.push(label.clone());
        game_state.ferris_index += 1;
        let ferris = engine.add_sprite(label.clone(), "cute_ferris.png");
        ferris.translation.x = thread_rng().gen_range(-550.0..550.0);
        ferris.translation.y = thread_rng().gen_range(-325.0..325.0);
        ferris.scale = 0.3;
        ferris.collision = true;
    }

    if engine.keyboard_state.just_pressed(KeyCode::R) {
        game_state.current_score = 0;
        let current_score = engine.texts.get_mut("current_score").unwrap();
        current_score.value = "Current score: 0".to_string(); 
    }

    if engine.keyboard_state.just_pressed(KeyCode::Q) {
        engine.should_exit = true;  
    }

    let offset = ((engine.time_since_startup_f64 * 3.0).cos() * 5.0) as f32;

    let current_score = engine.texts.get_mut("current_score").unwrap();
    current_score.translation.x = engine.window_dimensions.x / 2.0 - 110.0;
    current_score.translation.y = engine.window_dimensions.y / 2.0 - 30.0 + offset;

    let high_score = engine.texts.get_mut("high_score").unwrap();
    high_score.translation.x = -engine.window_dimensions.x / 2.0 + 110.0;
    high_score.translation.y = engine.window_dimensions.y / 2.0 - 30.0;
}
