mod ant;

use ggez::{Context, GameResult, ContextBuilder};
use ggez::event;
use ggez::graphics::{self, DrawMode, DrawParam, Rect, Mesh, spritebatch::SpriteBatch};

use ant::Ant;


const SCREEN_DIMS: (u32, u32) = (800, 800);
const GRID_SIZE: usize = 200;
const CELL_SIZE: f32 = SCREEN_DIMS.0 as f32/GRID_SIZE as f32;
const ANT_SPRITE_SIZE: f32 = 8.0;
const ANT_SCALE: f32 = CELL_SIZE/(ANT_SPRITE_SIZE * 2.0);

const STEP_PERIOD: f32 = 0.02;

struct MainState {
    grid: [[bool; GRID_SIZE]; GRID_SIZE],
    ant: Ant,

    black_cell_spritebatch: SpriteBatch,
    ant_spritebatch: SpriteBatch,
    update_timer: f32,
    step_count: usize,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let cell_mesh = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(0.0, 0.0, CELL_SIZE, CELL_SIZE),
            graphics::BLACK,
        )?;

        let ant_image = graphics::Image::new(ctx, "/assets/ant.png")?;
        let ant_spritebatch = SpriteBatch::new(ant_image);

        let cell_image = graphics::Image::new(ctx, "/assets/black_square.png")?;
        let black_cell_spritebatch = SpriteBatch::new(cell_image);
    
        let s = MainState {
            grid: [[true; GRID_SIZE]; GRID_SIZE],
            ant: Ant::default(),

            black_cell_spritebatch,
            ant_spritebatch,
            update_timer: 0.0,
            step_count: 0,
        };

        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        use ggez::timer;

        let dt = timer::duration_to_f64(timer::delta(ctx)) as f32;
        self.update_timer += dt;
        
        //if self.update_timer >= STEP_PERIOD {
        //    self.update_timer -= STEP_PERIOD;
            self.ant.step(&mut self.grid);
            self.step_count += 1;
        //}

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [1.0, 1.0, 1.0, 1.0].into());

        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                if !self.grid[x][y] {   // If black
                    self.black_cell_spritebatch.add(DrawParam::new()
                        .dest([
                            x as f32 * CELL_SIZE,
                            y as f32 * CELL_SIZE
                        ])
                        .scale([CELL_SIZE, CELL_SIZE])
                    );
                }
            }
        }

        // Draw ant
        self.ant_spritebatch.add(
            DrawParam::new()
                .dest([
                    (self.ant.pos.x as f32 + 0.5) * CELL_SIZE,
                    (self.ant.pos.y as f32 + 0.5) * CELL_SIZE
                ])
                .offset([0.5, 0.5])
                .rotation(self.ant.direction.in_radians())
                .scale([ANT_SCALE, ANT_SCALE])
        );

        graphics::draw(ctx, &self.black_cell_spritebatch, DrawParam::new())?;
        graphics::draw(ctx, &self.ant_spritebatch, DrawParam::new())?;
        self.ant_spritebatch.clear();
        self.black_cell_spritebatch.clear();

        // Draw step count
        let move_text = graphics::Text::new(format!("Steps: {}", self.step_count));
        graphics::draw(ctx, &move_text, DrawParam::new().dest([10.0, SCREEN_DIMS.1 as f32 - 20.0]).color([1.0, 0.0, 0.0, 1.0].into()))?;

        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult { 
    use ggez::conf::{WindowMode, WindowSetup, NumSamples};

    let mut cb = ContextBuilder::new("Langton's Ant", "eggmund")
        .window_mode(WindowMode::default()
            .dimensions(SCREEN_DIMS.0 as f32, SCREEN_DIMS.1 as f32)
        )
        .window_setup(WindowSetup::default()
            .samples(NumSamples::Zero)
        );

    if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let mut path = std::path::PathBuf::from(manifest_dir);
        path.push("resources");
        println!("Adding path {:?}", path);
        cb = cb.add_resource_path(path);
    }

    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new(ctx)?;
    event::run(ctx, event_loop, state)
}