use quartz::{Key, Context, Image, ShapeType, Canvas, GameObject, Action, Target, GameEvent, AnimatedSprite, CanvasMode, Location, Condition, Anchor};
use ramp::prism;
use prism::drawable::Drawable;

pub struct MyApp;

impl MyApp {
    fn new(ctx: &mut Context) -> impl Drawable {
        // Canvas setup
        let canvas_mode = CanvasMode::Landscape;
        let virtual_size = (3840.0, 2160.0);

        // Player dimensions and positioning
        let player_width = 200.0;
        let player_height = 330.0;
        let player_x = 400.0;
        let ground_level = virtual_size.1 - 350.0;
        let player_y = ground_level - player_height;

        // Load background image
        let bg_bytes = include_bytes!("../assets/background.png");
        let bg_img = image::load_from_memory(bg_bytes).expect("Failed to load background");
        let bg_image = bg_img.to_rgba8();

        // Create three background images for scrolling
        let background_image_1 = Image {
            shape: ShapeType::Rectangle(0.0, virtual_size, 0.0),
            image: bg_image.clone().into(),
            color: None
        };

        let background_image_2 = Image {
            shape: ShapeType::Rectangle(0.0, virtual_size, 0.0),
            image: bg_image.clone().into(),
            color: None
        };

        let background_image_3 = Image {
            shape: ShapeType::Rectangle(0.0, virtual_size, 0.0),
            image: bg_image.into(),
            color: None
        };

        // Load player idle animation
        let idle_bytes = include_bytes!("../assets/idle.gif");
        let idle_animation = AnimatedSprite::new(idle_bytes, (player_width, player_height), 8.0)
            .expect("Failed to load idle animation");
        let player_image = idle_animation.get_current_image();

        // Create companion (60% size of player)
        let companion_idle = AnimatedSprite::new(idle_bytes, (player_width * 0.6, player_height * 0.6), 8.0)
            .expect("Failed to load companion animation");
        let companion_image = companion_idle.get_current_image();

        // Create green indicator box (shows when running is allowed)
        let indicator_image = Image {
            shape: ShapeType::Rectangle(0.0, (50.0, 50.0), 0.0),
            image: image::RgbaImage::from_pixel(1, 1, image::Rgba([0, 255, 0, 255])).into(),
            color: None
        };

        let mut canvas = Canvas::new(ctx, canvas_mode);

        // Add first background panel
        let mut background_1 = GameObject::new_rect(
            ctx,
            "bg1".to_string(),
            background_image_1,
            virtual_size,
            (0.0, 0.0),
            vec!["background".to_string(), "scroll".to_string()],
            (0.0, 0.0),
            (1.0, 1.0),
            0.0,
        );
        background_1.scaled_size.set(virtual_size);
        background_1.update_image_shape();
        canvas.add_game_object("background1".to_string(), background_1);

        // Add second background panel (positioned to the right)
        let mut background_2 = GameObject::new_rect(
            ctx,
            "bg2".to_string(),
            background_image_2,
            virtual_size,
            (virtual_size.0, 0.0),
            vec!["background".to_string(), "scroll".to_string()],
            (0.0, 0.0),
            (1.0, 1.0),
            0.0,
        );
        background_2.scaled_size.set(virtual_size);
        background_2.update_image_shape();
        canvas.add_game_object("background2".to_string(), background_2);

        // Add third background panel
        let mut background_3 = GameObject::new_rect(
            ctx,
            "bg3".to_string(),
            background_image_3,
            virtual_size,
            (virtual_size.0 * 2.0, 0.0),
            vec!["background".to_string(), "scroll".to_string()],
            (0.0, 0.0),
            (1.0, 1.0),
            0.0,
        );
        background_3.scaled_size.set(virtual_size);
        background_3.update_image_shape();
        canvas.add_game_object("background3".to_string(), background_3);

        // Create ground platform
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

        // Add main player character
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

        // Add companion character
        let companion_width = player_width * 0.6;
        let companion_height = player_height * 0.6;
        let companion = GameObject::new_rect(
            ctx,
            "companion".to_string(),
            companion_image,
            (companion_width, companion_height),
            (player_x + 250.0, player_y),
            vec!["companion".to_string()],
            (0.0, 0.0),
            (0.98, 0.98),
            1.2,
        ).with_animation(companion_idle);
        canvas.add_game_object("companion".to_string(), companion);

        // Add running indicator
        let mut indicator = GameObject::new_rect(
            ctx,
            "run_indicator".to_string(),
            indicator_image,
            (50.0, 50.0),
            (100.0, 100.0),
            vec!["indicator".to_string()],
            (0.0, 0.0),
            (1.0, 1.0),
            0.0,
        );
        indicator.scaled_size.set((50.0, 50.0));
        indicator.update_image_shape();
        canvas.add_game_object("run_indicator".to_string(), indicator);

        // W key: Start walking - player animation
        canvas.add_event(
            GameEvent::KeyPress {
                key: Key::Character("w".to_string().into()),
                action: Action::SetAnimation {
                    target: Target::ById("player".to_string()),
                    animation_bytes: include_bytes!("../assets/walking.gif"),
                    fps: 16.0,
                },
                target: Target::ById("player".to_string())
            },
            Target::ById("player".to_string())
        );

        // W key: Start walking - companion animation (only if visible)
        canvas.add_event(
            GameEvent::KeyPress {
                key: Key::Character("w".to_string().into()),
                action: Action::Conditional {
                    condition: Condition::IsVisible(Target::ById("companion".to_string())),
                    if_true: Box::new(Action::SetAnimation {
                        target: Target::ById("companion".to_string()),
                        animation_bytes: include_bytes!("../assets/walking.gif"),
                        fps: 16.0,
                    }),
                    if_false: None,
                },
                target: Target::ById("companion".to_string())
            },
            Target::ById("companion".to_string())
        );

        // W key: Scroll background for walking effect
        canvas.add_event(
            GameEvent::KeyPress {
                key: Key::Character("w".to_string().into()),
                action: Action::SetMomentum {
                    target: Target::ByTag("background".to_string()),
                    value: (-8.0, 0.0)
                },
                target: Target::ById("player".to_string())
            },
            Target::ById("player".to_string())
        );

        // W key release: Return to idle - player
        canvas.add_event(
            GameEvent::KeyRelease {
                key: Key::Character("w".to_string().into()),
                action: Action::SetAnimation {
                    target: Target::ById("player".to_string()),
                    animation_bytes: include_bytes!("../assets/idle.gif"),
                    fps: 8.0,
                },
                target: Target::ById("player".to_string())
            },
            Target::ById("player".to_string())
        );

        // W key release: Return to idle - companion (only if visible)
        canvas.add_event(
            GameEvent::KeyRelease {
                key: Key::Character("w".to_string().into()),
                action: Action::Conditional {
                    condition: Condition::IsVisible(Target::ById("companion".to_string())),
                    if_true: Box::new(Action::SetAnimation {
                        target: Target::ById("companion".to_string()),
                        animation_bytes: include_bytes!("../assets/idle.gif"),
                        fps: 8.0,
                    }),
                    if_false: None,
                },
                target: Target::ById("companion".to_string())
            },
            Target::ById("companion".to_string())
        );

        // W key release: Stop background scrolling
        canvas.add_event(
            GameEvent::KeyRelease {
                key: Key::Character("w".to_string().into()),
                action: Action::SetMomentum {
                    target: Target::ByTag("background".to_string()),
                    value: (0.0, 0.0)
                },
                target: Target::ById("player".to_string())
            },
            Target::ById("player".to_string())
        );

        // R key: Start running - player (only if indicator is visible)
        canvas.add_event(
            GameEvent::KeyPress {
                key: Key::Character("r".to_string().into()),
                action: Action::Conditional {
                    condition: Condition::IsVisible(Target::ById("run_indicator".to_string())),
                    if_true: Box::new(Action::SetAnimation {
                        target: Target::ById("player".to_string()),
                        animation_bytes: include_bytes!("../assets/running.gif"),
                        fps: 16.0,
                    }),
                    if_false: None,
                },
                target: Target::ById("player".to_string())
            },
            Target::ById("player".to_string())
        );

        // R key: Start running - companion (only if both indicator AND companion are visible)
        canvas.add_event(
            GameEvent::KeyPress {
                key: Key::Character("r".to_string().into()),
                action: Action::Conditional {
                    condition: Condition::And(
                        Box::new(Condition::IsVisible(Target::ById("run_indicator".to_string()))),
                        Box::new(Condition::IsVisible(Target::ById("companion".to_string())))
                    ),
                    if_true: Box::new(Action::SetAnimation {
                        target: Target::ById("companion".to_string()),
                        animation_bytes: include_bytes!("../assets/running.gif"),
                        fps: 16.0,
                    }),
                    if_false: None,
                },
                target: Target::ById("companion".to_string())
            },
            Target::ById("companion".to_string())
        );

        // R key: Faster background scroll for running (only if allowed)
        canvas.add_event(
            GameEvent::KeyPress {
                key: Key::Character("r".to_string().into()),
                action: Action::Conditional {
                    condition: Condition::IsVisible(Target::ById("run_indicator".to_string())),
                    if_true: Box::new(Action::SetMomentum {
                        target: Target::ByTag("background".to_string()),
                        value: (-20.0, 0.0)
                    }),
                    if_false: None,
                },
                target: Target::ById("player".to_string())
            },
            Target::ById("player".to_string())
        );

        // R key release: Return to idle - player
        canvas.add_event(
            GameEvent::KeyRelease {
                key: Key::Character("r".to_string().into()),
                action: Action::SetAnimation {
                    target: Target::ById("player".to_string()),
                    animation_bytes: include_bytes!("../assets/idle.gif"),
                    fps: 8.0,
                },
                target: Target::ById("player".to_string())
            },
            Target::ById("player".to_string())
        );

        // R key release: Return to idle - companion (only if visible)
        canvas.add_event(
            GameEvent::KeyRelease {
                key: Key::Character("r".to_string().into()),
                action: Action::Conditional {
                    condition: Condition::IsVisible(Target::ById("companion".to_string())),
                    if_true: Box::new(Action::SetAnimation {
                        target: Target::ById("companion".to_string()),
                        animation_bytes: include_bytes!("../assets/idle.gif"),
                        fps: 8.0,
                    }),
                    if_false: None,
                },
                target: Target::ById("companion".to_string())
            },
            Target::ById("companion".to_string())
        );

        // R key release: Stop background scrolling
        canvas.add_event(
            GameEvent::KeyRelease {
                key: Key::Character("r".to_string().into()),
                action: Action::SetMomentum {
                    target: Target::ByTag("background".to_string()),
                    value: (0.0, 0.0)
                },
                target: Target::ById("player".to_string())
            },
            Target::ById("player".to_string())
        );

        // A key: Jump - player animation
        canvas.add_event(
            GameEvent::KeyPress {
                key: Key::Character("a".to_string().into()),
                action: Action::SetAnimation {
                    target: Target::ById("player".to_string()),
                    animation_bytes: include_bytes!("../assets/jumping.gif"),
                    fps: 10.0,
                },
                target: Target::ById("player".to_string())
            },
            Target::ById("player".to_string())
        );

        // A key: Jump - companion animation (only if visible)
        canvas.add_event(
            GameEvent::KeyPress {
                key: Key::Character("a".to_string().into()),
                action: Action::Conditional {
                    condition: Condition::IsVisible(Target::ById("companion".to_string())),
                    if_true: Box::new(Action::SetAnimation {
                        target: Target::ById("companion".to_string()),
                        animation_bytes: include_bytes!("../assets/jumping.gif"),
                        fps: 10.0,
                    }),
                    if_false: None,
                },
                target: Target::ById("companion".to_string())
            },
            Target::ById("companion".to_string())
        );

        // A key: Apply upward momentum - player
        canvas.add_event(
            GameEvent::KeyPress {
                key: Key::Character("a".to_string().into()),
                action: Action::ApplyMomentum {
                    target: Target::ById("player".to_string()),
                    value: (0.0, -30.0)
                },
                target: Target::ById("player".to_string())
            },
            Target::ById("player".to_string())
        );

        // A key: Apply upward momentum - companion (only if visible)
        canvas.add_event(
            GameEvent::KeyPress {
                key: Key::Character("a".to_string().into()),
                action: Action::Conditional {
                    condition: Condition::IsVisible(Target::ById("companion".to_string())),
                    if_true: Box::new(Action::ApplyMomentum {
                        target: Target::ById("companion".to_string()),
                        value: (0.0, -30.0)
                    }),
                    if_false: None,
                },
                target: Target::ById("companion".to_string())
            },
            Target::ById("companion".to_string())
        );

        // O key: Toggle running permission indicator
        canvas.add_event(
            GameEvent::KeyPress {
                key: Key::Character("o".to_string().into()),
                action: Action::Toggle {
                    target: Target::ById("run_indicator".to_string()),
                },
                target: Target::ById("player".to_string())
            },
            Target::ById("player".to_string())
        );

        // Toggle companion visibility every 5 seconds
        let mut tick_counter = 0u32;
        canvas.on_tick(move |canvas| {
            tick_counter += 1;
            
            if tick_counter >= 300 {
                tick_counter = 0;
                
                if let Some(companion) = canvas.get_game_object_mut("companion") {
                    companion.visible = !companion.visible;
                    
                    // Reset momentum when toggling visibility to prevent buildup
                    companion.momentum = (0.0, 0.0);
                }
            }
        });


        canvas
    }
}

ramp::run!{|ctx: &mut Context| {
    MyApp::new(ctx)
}}