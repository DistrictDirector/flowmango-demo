use flowmango::{Key,Context, Image, ShapeType, Canvas, GameObject, Action, Target, GameEvent, AnimatedSprite, CanvasMode};
use ramp::prism;

pub struct MyApp;

impl MyApp {
    fn new(ctx: &mut Context) -> impl Drawable {

        let canvas_mode = CanvasMode::Portrait; 
        
        let virtual_size = match canvas_mode {
            CanvasMode::Landscape => (3840.0, 2160.0),
            CanvasMode::Portrait => (2160.0, 3840.0),
        };
        

        let flappybird_width = 50.0;
        let flappybird_height = 35.0;

        let bg_bytes = include_bytes!("../assets/bg.png");
        let bg_img = image::load_from_memory(bg_bytes)
            .expect("Failed to load background image");
        let bg_image = bg_img.to_rgba8();
        
        let mut background_image = Image {
            shape: ShapeType::Rectangle(0.0, virtual_size, 0.0),
            image: bg_image.into(),
            color: None
        };

        let flappybird_gif_bytes = include_bytes!("../assets/flappybird.gif");
        let flappybird_animation = AnimatedSprite::new(
            flappybird_gif_bytes,
            (flappybird_width, flappybird_height),
            12.0  
        ).expect("Failed to load flappy bird animation");

        let flappybird_image = flappybird_animation.get_current_image();

        let mut stork_canvas = Canvas::new(ctx, canvas_mode);

        let mut background = GameObject::new_rect(
            ctx,
            "background".to_string(),
            background_image,
            virtual_size,
            (0.0, 0.0),
            vec!["background".to_string()],
            (0.0, 0.0),
            (1.0, 1.0),
            0.0,
        );
        
        background.scaled_size.set(virtual_size);
        background.update_image_shape();

        stork_canvas.add_game_object("background".to_string(), background);
        
        let flappybird = GameObject::new_rect(
            ctx,
            "flappybird".to_string(),
            flappybird_image,
            (flappybird_width, flappybird_height),
            (200.0, 300.0),
            vec![
                "player".to_string(),
                "flyingbird".to_string(),
            ],
            (0.0, 0.0),
            (0.85, 0.85), 
            0.30,
        )
        .with_animation(flappybird_animation);

        stork_canvas.add_game_object("flappybird".to_string(), flappybird);

        stork_canvas.add_event(
            GameEvent::KeyPress {
                key: Key::Character("w".to_string().into()),
                action: Action::ApplyMomentum {  
                    target: Target::ById("flappybird".to_string()),
                    value: (0.0, -10.5)  
                },
                target: Target::ById("flappybird".to_string())
            },
            Target::ById("flappybird".to_string())
        );


        stork_canvas.add_event(
            GameEvent::BoundaryCollision {
                action: Action::SetMomentum {
                    target: Target::ById("flappybird".to_string()),
                    value: (0.0, 0.0)
                },
                target: Target::ById("flappybird".to_string())
            },
            Target::ById("flappybird".to_string())
        );

        stork_canvas
    }
}

ramp::run!{|ctx: &mut Context| {
    MyApp::new(ctx)
}}