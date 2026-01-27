use flowmango::{Key, Context, Image, ShapeType, Canvas, GameObject, Action, Target, GameEvent, AnimatedSprite, CanvasMode, Location, Condition, Anchor};
use ramp::prism;
use prism::drawable::Drawable;

pub struct MyApp;

impl MyApp {
    fn new(ctx: &mut Context) -> impl Drawable {
        let canvas_mode = CanvasMode::Landscape;
        let virtual_size = (3840.0, 2160.0);

        let player_width = 200.0;
        let player_height = 330.0;
        let player_x = 400.0;
        let ground_level = virtual_size.1 - 350.0;
        let player_y = ground_level - player_height;

        let bg_bytes = include_bytes!("../assets/background.png");
        let bg_img = image::load_from_memory(bg_bytes).expect("Failed to load background");
        let bg_image = bg_img.to_rgba8();

        let background_image = Image {
            shape: ShapeType::Rectangle(0.0, virtual_size, 0.0),
            image: bg_image.into(),
            color: None
        };

        let idle_bytes = include_bytes!("../assets/idle.gif");
        let idle_animation = AnimatedSprite::new(idle_bytes, (player_width, player_height), 8.0)
            .expect("Failed to load idle animation");
        let player_image = idle_animation.get_current_image();

        // Create green block image for anchor markers
        let block_size = 40.0;
        let green_block_image = Image {
            shape: ShapeType::Rectangle(0.0, (block_size, block_size), 0.0),
            image: image::RgbaImage::from_pixel(1, 1, 
                image::Rgba([0, 255, 0, 255])).into(),
            color: None
        };

        // Create red block image for custom anchor markers
        let red_block_image = Image {
            shape: ShapeType::Rectangle(0.0, (block_size, block_size), 0.0),
            image: image::RgbaImage::from_pixel(1, 1, 
                image::Rgba([255, 0, 0, 255])).into(),
            color: None
        };

        // Create blue block image for outside anchor markers
        let blue_block_image = Image {
            shape: ShapeType::Rectangle(0.0, (block_size, block_size), 0.0),
            image: image::RgbaImage::from_pixel(1, 1, 
                image::Rgba([0, 100, 255, 255])).into(),
            color: None
        };

        let mut canvas = Canvas::new(ctx, canvas_mode);

        let mut background = GameObject::new_rect(
            ctx,
            "bg".to_string(),
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
        canvas.add_game_object("background".to_string(), background);

        let ground_image = Image {
            shape: ShapeType::Rectangle(0.0, (virtual_size.0 * 10.0, 10.0), 0.0),
            image: image::RgbaImage::new(1, 1).into(),
            color: None
        };

        let mut ground = GameObject::new_rect(
            ctx,
            "ground".to_string(),
            ground_image,
            (virtual_size.0 * 10.0, 10.0),
            (-virtual_size.0 * 2.0, ground_level),
            vec!["ground".to_string(), "platform".to_string(), "background".to_string()],
            (0.0, 0.0),
            (1.0, 1.0),
            0.0,
        ).as_platform();
        ground.scaled_size.set((virtual_size.0 * 10.0, 10.0));
        ground.update_image_shape();
        canvas.add_game_object("ground".to_string(), ground);

        let player = GameObject::new_rect(
            ctx,
            "player".to_string(),
            player_image,
            (player_width, player_height),
            (player_x, player_y),
            vec!["player".to_string()],
            (0.0, 0.0),
            (0.98, 0.98),
            1.2,
        ).with_animation(idle_animation);
        canvas.add_game_object("player".to_string(), player);

        // Define standard anchor positions with their names (GREEN blocks)
        let standard_anchors = vec![
            ("TopLeft", Anchor { x: 0.0, y: 0.0 }),
            ("TopCenter", Anchor { x: 0.5, y: 0.0 }),
            ("TopRight", Anchor { x: 1.0, y: 0.0 }),
            ("CenterLeft", Anchor { x: 0.0, y: 0.5 }),
            ("Center", Anchor { x: 0.5, y: 0.5 }),
            ("CenterRight", Anchor { x: 1.0, y: 0.5 }),
            ("BottomLeft", Anchor { x: 0.0, y: 1.0 }),
            ("BottomCenter", Anchor { x: 0.5, y: 1.0 }),
            ("BottomRight", Anchor { x: 1.0, y: 1.0 }),
        ];

        // Create green blocks at standard anchor positions
        for (name, anchor) in standard_anchors {
            let block = GameObject::new(
                ctx,
                format!("anchor_{}", name),
                green_block_image.clone(),
                block_size,
                (0.0, 0.0),
                vec!["anchor_marker".to_string()],
                (0.0, 0.0),
                (1.0, 1.0),
                0.0,
            );
            canvas.add_game_object(format!("anchor_{}", name), block);

            canvas.run(Action::Teleport {
                target: Target::ById(format!("anchor_{}", name)),
                location: Location::OnTarget {
                    target: Box::new(Target::ById("player".to_string())),
                    anchor: anchor,
                    offset: (0.0, 0.0),
                }
            });
        }

        // Define custom anchor positions INSIDE the player (RED blocks)
        // Using normalized coordinates (0.0 to 1.0)
        let custom_anchors_inside = vec![
            ("Custom_Quarter_Quarter", 0.25, 0.25),      // 25% from left, 25% from top
            ("Custom_ThreeQuarter_Quarter", 0.75, 0.25), // 75% from left, 25% from top
            ("Custom_Quarter_ThreeQuarter", 0.25, 0.75), // 25% from left, 75% from top
            ("Custom_ThreeQuarter_ThreeQuarter", 0.75, 0.75), // 75% from left, 75% from top
            ("Custom_Third_Half", 0.33, 0.5),            // 33% from left, centered vertically
            ("Custom_TwoThirds_Half", 0.67, 0.5),        // 67% from left, centered vertically
        ];

        // Create RED blocks at custom positions INSIDE the player
        for (name, x, y) in custom_anchors_inside {
            let block = GameObject::new(
                ctx,
                format!("anchor_{}", name),
                red_block_image.clone(),
                block_size,
                (0.0, 0.0),
                vec!["anchor_marker".to_string()],
                (0.0, 0.0),
                (1.0, 1.0),
                0.0,
            );
            canvas.add_game_object(format!("anchor_{}", name), block);

            canvas.run(Action::Teleport {
                target: Target::ById(format!("anchor_{}", name)),
                location: Location::OnTarget {
                    target: Box::new(Target::ById("player".to_string())),
                    anchor: Anchor { x, y },
                    offset: (0.0, 0.0),
                }
            });
        }

        // Define custom anchor positions OUTSIDE the player (BLUE blocks)
        // Using values < 0.0 or > 1.0 to position outside the object
        let custom_anchors_outside = vec![
            ("Custom_Left_Outside", -0.5, 0.5),          // 50% to the left, centered vertically
            ("Custom_Right_Outside", 1.5, 0.5),          // 50% to the right, centered vertically
            ("Custom_Top_Outside", 0.5, -0.5),           // Centered horizontally, 50% above
            ("Custom_Bottom_Outside", 0.5, 1.5),         // Centered horizontally, 50% below
            ("Custom_TopLeft_Outside", -0.3, -0.3),      // Diagonal top-left outside
            ("Custom_BottomRight_Outside", 1.3, 1.3),    // Diagonal bottom-right outside
        ];

        // Create BLUE blocks at custom positions OUTSIDE the player
        for (name, x, y) in custom_anchors_outside {
            let block = GameObject::new(
                ctx,
                format!("anchor_{}", name),
                blue_block_image.clone(),
                block_size,
                (0.0, 0.0),
                vec!["anchor_marker".to_string()],
                (0.0, 0.0),
                (1.0, 1.0),
                0.0,
            );
            canvas.add_game_object(format!("anchor_{}", name), block);

            canvas.run(Action::Teleport {
                target: Target::ById(format!("anchor_{}", name)),
                location: Location::OnTarget {
                    target: Box::new(Target::ById("player".to_string())),
                    anchor: Anchor { x, y },
                    offset: (0.0, 0.0),
                }
            });
        }

        canvas
    }
}

ramp::run!{|ctx: &mut Context| {
    MyApp::new(ctx)
}}