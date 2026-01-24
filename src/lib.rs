// ============================================================================
// FLOWMANGO 2D PLATFORMER GAME - COMPREHENSIVE GUIDE
// ============================================================================
// This is a side-scrolling platformer game where the player can walk, run,
// and jump. The background scrolls to create the illusion of forward movement.
// ============================================================================

use flowmango::{Key, Context, Image, ShapeType, Canvas, GameObject, Action, Target, GameEvent, AnimatedSprite, CanvasMode, Location};
use ramp::prism;
use prism::drawable::Drawable;

pub struct MyApp;

impl MyApp {
    fn new(ctx: &mut Context) -> impl Drawable {
        
        // ====================================================================
        // SECTION 1: CANVAS SETUP AND DIMENSIONS
        // ====================================================================
        
        // CanvasMode::Landscape - Sets the game to landscape orientation
        // This determines how the game scales and displays on different screens
        // Other options: CanvasMode::Portrait, CanvasMode::Square
        let canvas_mode = CanvasMode::Landscape;
        
        // virtual_size - The internal resolution of your game world
        // (3840.0, 2160.0) = 4K resolution (width, height)
        // All game objects use these coordinates as reference
        // The actual window can be any size - this will scale accordingly
        let virtual_size = (3840.0, 2160.0);

        // ====================================================================
        // SECTION 2: PLAYER DIMENSIONS AND POSITIONING
        // ====================================================================
        
        // Player sprite dimensions - must match your sprite sheet size
        let player_width = 200.0;   // Width of each frame in the sprite animation
        let player_height = 330.0;  // Height of each frame (30 pixels added for walking animation)
        
        // Player X position - where the player appears horizontally
        // 400.0 = 400 pixels from the left edge of the screen
        let player_x = 400.0;
        
        // Ground level calculation
        // virtual_size.1 = 2160.0 (screen height)
        // 2160.0 - 350.0 = 1810.0 (where the ground starts)
        let ground_level = virtual_size.1 - 350.0;
        
        // Player Y position - calculated to place player on the ground
        // 1810.0 - 330.0 = 1480.0 (top of player sprite)
        let player_y = ground_level - player_height;

        // ====================================================================
        // SECTION 3: LOADING AND CREATING BACKGROUND IMAGES
        // ====================================================================
        
        // include_bytes! - Embeds the image file directly into the compiled binary
        // This macro reads the file at compile time, so the image is part of your .exe
        // Path is relative to your Cargo.toml file
        let bg_bytes = include_bytes!("../assets/background.png");
        
        // image::load_from_memory - Decodes the PNG bytes into an image object
        // .expect() - Crashes with an error message if loading fails
        let bg_img = image::load_from_memory(bg_bytes).expect("Failed to load background");
        
        // .to_rgba8() - Converts image to RGBA format (Red, Green, Blue, Alpha)
        // This ensures consistent color handling with 8 bits per channel
        let bg_image = bg_img.to_rgba8();
        
        // Image struct - Defines how an image is displayed
        // This creates the first background instance
        let background_image_1 = Image {
            // ShapeType::Rectangle defines the drawable area
            // Parameters: (rotation, (width, height), z-index)
            // 0.0 = no rotation
            // virtual_size = full screen size (3840, 2160)
            // 0.0 = z-index (depth layer, 0 is back)
            shape: ShapeType::Rectangle(0.0, virtual_size, 0.0),
            
            // .clone() creates a copy of the image data
            // .into() converts it to the required type
            image: bg_image.clone().into(),
            
            // color: None means no color tinting is applied
            // If set to Some((255, 0, 0, 255)), it would tint red
            color: None
        };
        
        // Create second background (for seamless scrolling)
        let background_image_2 = Image {
            shape: ShapeType::Rectangle(0.0, virtual_size, 0.0),
            image: bg_image.clone().into(),
            color: None
        };
        
        // Create third background (for seamless scrolling)
        // No .clone() here because this is the last use of bg_image
        let background_image_3 = Image {
            shape: ShapeType::Rectangle(0.0, virtual_size, 0.0),
            image: bg_image.into(),
            color: None
        };

        // ====================================================================
        // SECTION 4: LOADING PLAYER ANIMATION
        // ====================================================================
        
        // Load the idle animation GIF file
        let idle_bytes = include_bytes!("../assets/idle.gif");
        
        // AnimatedSprite::new - Creates an animated sprite from a GIF
        // Parameters:
        //   - idle_bytes: The GIF file data
        //   - (player_width, player_height): Size to display each frame
        //   - 8.0: FPS (frames per second) - how fast the animation plays
        let idle_animation = AnimatedSprite::new(idle_bytes, (player_width, player_height), 8.0)
            .expect("Failed to load idle animation");
        
        // Get the first frame of the animation to use as initial player image
        let player_image = idle_animation.get_current_image();

        // ====================================================================
        // SECTION 5: CREATE THE CANVAS (GAME WORLD CONTAINER)
        // ====================================================================
        
        // Canvas - The main container that holds all game objects
        // ctx: The graphics context (handles rendering)
        // canvas_mode: Landscape orientation we defined earlier
        let mut canvas = Canvas::new(ctx, canvas_mode);

        // ====================================================================
        // SECTION 6: CREATE BACKGROUND GAME OBJECTS (FOR SCROLLING)
        // ====================================================================
        
        // GameObject::new_rect - Creates a rectangular game object
        // This is the FIRST background image (leftmost)
        //
        // Parameters explained:
        let mut background_1 = GameObject::new_rect(
            ctx,                              // Graphics context
            "bg1".to_string(),                // Internal ID (not displayed)
            background_image_1,               // The image to display
            virtual_size,                     // Size: (3840, 2160)
            (0.0, 0.0),                       // Position: top-left corner
            
            // Tags - Used to target multiple objects at once
            // "background" tag: Used for general background operations
            // "scroll" tag: Used to apply scrolling movement
            vec!["background".to_string(), "scroll".to_string()],
            
            (0.0, 0.0),                       // Momentum: (x_speed, y_speed) - starts stationary
            (1.0, 1.0),                       // Friction: (x_friction, y_friction) - no slowdown
            0.0,                              // Gravity: 0.0 = no gravity (backgrounds don't fall)
        );
        
        // .scaled_size.set() - Ensures the background fills the entire screen
        background_1.scaled_size.set(virtual_size);
        
        // .update_image_shape() - Updates the internal shape after size changes
        // Must be called after modifying scaled_size
        background_1.update_image_shape();
        
        // Add the background to the canvas with a unique string ID
        canvas.add_game_object("background1".to_string(), background_1);

        // SECOND BACKGROUND - Positioned to the right of the first
        // This creates seamless scrolling when bg1 moves left
        let mut background_2 = GameObject::new_rect(
            ctx,
            "bg2".to_string(),
            background_image_2,
            virtual_size,
            // Position: (3840, 0) - starts right next to bg1
            // When bg1 is at x=0, bg2 is at x=3840 (just off screen to the right)
            (virtual_size.0, 0.0),
            vec!["background".to_string(), "scroll".to_string()],
            (0.0, 0.0),
            (1.0, 1.0),
            0.0,
        );
        background_2.scaled_size.set(virtual_size);
        background_2.update_image_shape();
        canvas.add_game_object("background2".to_string(), background_2);

        // THIRD BACKGROUND - Positioned to the right of the second
        let mut background_3 = GameObject::new_rect(
            ctx,
            "bg3".to_string(),
            background_image_3,
            virtual_size,
            // Position: (7680, 0) - 3840 * 2
            // This is two screen widths to the right
            (virtual_size.0 * 2.0, 0.0),
            vec!["background".to_string(), "scroll".to_string()],
            (0.0, 0.0),
            (1.0, 1.0),
            0.0,
        );
        background_3.scaled_size.set(virtual_size);
        background_3.update_image_shape();
        canvas.add_game_object("background3".to_string(), background_3);

        // ====================================================================
        // SECTION 7: CREATE GROUND PLATFORM (FOR COLLISION)
        // ====================================================================
        
        // Create a simple ground image (invisible but collidable)
        let ground_image = Image {
            // Ground is very wide (10x screen width) and thin (10 pixels tall)
            shape: ShapeType::Rectangle(0.0, (virtual_size.0 * 10.0, 10.0), 0.0),
            
            // RgbaImage::new(1, 1) - Creates a 1x1 pixel transparent image
            // We don't need a visible image, just collision detection
            image: image::RgbaImage::new(1, 1).into(),
            color: None
        };
        
        // Create the ground platform GameObject
        let mut ground = GameObject::new_rect(
            ctx,
            "ground".to_string(),
            ground_image,
            (virtual_size.0 * 10.0, 10.0),    // Very wide platform
            
            // Position: starts far to the left (-2 screen widths)
            // This ensures there's always ground under the player even when scrolling
            (-virtual_size.0 * 2.0, ground_level),
            
            // Tags: "ground", "platform" for collision, "background" for scrolling
            vec!["ground".to_string(), "platform".to_string(), "background".to_string()],
            (0.0, 0.0),
            (1.0, 1.0),
            0.0,
        ).as_platform();  // .as_platform() - Makes this object solid for collisions
        
        ground.scaled_size.set((virtual_size.0 * 10.0, 10.0));
        ground.update_image_shape();
        canvas.add_game_object("ground".to_string(), ground);

        // ====================================================================
        // SECTION 8: CREATE PLAYER GAME OBJECT
        // ====================================================================
        
        let player = GameObject::new_rect(
            ctx,
            "player".to_string(),
            player_image,                     // First frame of idle animation
            (player_width, player_height),    // (200, 330)
            (player_x, player_y),             // (400, 1480) - calculated earlier
            vec!["player".to_string()],       // Tag for targeting
            (0.0, 0.0),                       // No initial momentum
            
            // Friction: (0.98, 0.98) - Slight slowdown each frame
            // 1.0 = no friction, 0.0 = instant stop
            // 0.98 = retains 98% of speed each frame (gradual slowdown)
            (0.98, 0.98),
            
            // Gravity: 1.2 - How fast the player falls
            // Higher = falls faster, 0 = no gravity
            1.2,
        ).with_animation(idle_animation);  // Attach the idle animation
        
        canvas.add_game_object("player".to_string(), player);

        // ====================================================================
        // SECTION 9: W KEY - WALKING CONTROLS
        // ====================================================================
        
        // GameEvent::KeyPress - Triggers when a key is pressed DOWN
        canvas.add_event(
            GameEvent::KeyPress {
                // Key to detect: 'w' key
                key: Key::Character("w".to_string().into()),
                
                // Action::SetAnimation - Changes the player's animation
                action: Action::SetAnimation {
                    // Target::ById - Targets the object with ID "player"
                    target: Target::ById("player".to_string()),
                    
                    // Load the walking animation GIF
                    animation_bytes: include_bytes!("../assets/walking.gif"),
                    
                    // fps: 16.0 - Animation plays at 16 frames per second
                    // Higher = faster animation
                    fps: 16.0,
                },
                target: Target::ById("player".to_string())
            },
            Target::ById("player".to_string())  // Which object listens for this event
        );
        
        // Second event for W key press: Move the background
        canvas.add_event(
            GameEvent::KeyPress {
                key: Key::Character("w".to_string().into()),
                
                // Action::SetMomentum - Sets the speed of objects
                action: Action::SetMomentum {
                    // Target::ByTag - Targets ALL objects with "background" tag
                    // This affects bg1, bg2, bg3, and ground simultaneously
                    target: Target::ByTag("background".to_string()),
                    
                    // value: (-8.0, 0.0) - Momentum vector (x, y)
                    // -8.0 = move LEFT at 8 pixels per frame
                    // 0.0 = no vertical movement
                    // Negative X = moves left (creates illusion player moves right)
                    value: (-8.0, 0.0)
                },
                target: Target::ById("player".to_string())
            },
            Target::ById("player".to_string())
        );
        
        // GameEvent::KeyRelease - Triggers when the key is RELEASED
        canvas.add_event(
            GameEvent::KeyRelease {
                key: Key::Character("w".to_string().into()),
                
                // Return to idle animation when W is released
                action: Action::SetAnimation {
                    target: Target::ById("player".to_string()),
                    animation_bytes: include_bytes!("../assets/idle.gif"),
                    fps: 8.0,  // Slower FPS for idle (more relaxed)
                },
                target: Target::ById("player".to_string())
            },
            Target::ById("player".to_string())
        );
        
        // Stop background movement when W is released
        canvas.add_event(
            GameEvent::KeyRelease {
                key: Key::Character("w".to_string().into()),
                
                action: Action::SetMomentum {
                    target: Target::ByTag("background".to_string()),
                    
                    // value: (0.0, 0.0) - Stop all movement
                    value: (0.0, 0.0)
                },
                target: Target::ById("player".to_string())
            },
            Target::ById("player".to_string())
        );

        // ====================================================================
        // SECTION 10: R KEY - RUNNING CONTROLS
        // ====================================================================
        
        // Running is similar to walking but FASTER
        canvas.add_event(
            GameEvent::KeyPress {
                key: Key::Character("r".to_string().into()),
                
                // Load running animation when R is pressed
                action: Action::SetAnimation {
                    target: Target::ById("player".to_string()),
                    animation_bytes: include_bytes!("../assets/running.gif"),
                    fps: 16.0,
                },
                target: Target::ById("player".to_string())
            },
            Target::ById("player".to_string())
        );
        
        // Move background FASTER when running
        canvas.add_event(
            GameEvent::KeyPress {
                key: Key::Character("r".to_string().into()),
                
                action: Action::SetMomentum {
                    target: Target::ByTag("background".to_string()),
                    
                    // value: (-20.0, 0.0) - Much faster than walking (-8.0)
                    // This creates the running speed effect
                    value: (-20.0, 0.0)
                },
                target: Target::ById("player".to_string())
            },
            Target::ById("player".to_string())
        );
        
        // Return to idle when R is released
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
        
        // Stop background movement when R is released
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

        // ====================================================================
        // SECTION 11: A KEY - JUMPING CONTROLS
        // ====================================================================
        
        // Change to jumping animation when A is pressed
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
        
        // Apply upward momentum to make the player jump
        canvas.add_event(
            GameEvent::KeyPress {
                key: Key::Character("a".to_string().into()),
                
                // Action::ApplyMomentum - ADDS to existing momentum
                // Different from SetMomentum which replaces it
                action: Action::ApplyMomentum {
                    target: Target::ById("player".to_string()),
                    
                    // value: (0.0, -30.0)
                    // 0.0 = no horizontal push
                    // -30.0 = strong upward push (negative Y = up)
                    // This creates the jump arc (up fast, then gravity pulls down)
                    value: (0.0, -30.0)
                },
                target: Target::ById("player".to_string())
            },
            Target::ById("player".to_string())
        );

        // ====================================================================
        // SECTION 12: RETURN THE CANVAS TO START THE GAME
        // ====================================================================
        
        // Return the canvas - this starts rendering the game
        canvas
    }
}

// ========================================================================
// SECTION 13: GAME ENTRY POINT
// ========================================================================

// ramp::run! macro - The main entry point that launches the game
// |ctx: &mut Context| - Provides the graphics context
// MyApp::new(ctx) - Creates and initializes the game
ramp::run!{|ctx: &mut Context| {
    MyApp::new(ctx)
}}

// ========================================================================
// KEY CONCEPTS SUMMARY:
// ========================================================================
//
// 1. CANVAS: The game world container that holds all objects
//
// 2. GAME OBJECTS: Visual elements (player, backgrounds, platforms)
//    - Created with GameObject::new_rect()
//    - Each has position, size, momentum, friction, gravity
//    - Can have animations attached
//
// 3. TAGS: String labels used to target multiple objects at once
//    - Target::ByTag("background") affects all objects with that tag
//    - Target::ById("player") affects only the specific player object
//
// 4. MOMENTUM: (x, y) speed vector
//    - SetMomentum: Replaces current speed
//    - ApplyMomentum: Adds to current speed
//    - Negative X = left, Positive X = right
//    - Negative Y = up, Positive Y = down
//
// 5. FRICTION: How much speed is retained each frame
//    - 1.0 = no slowdown (100% speed kept)
//    - 0.98 = gradual slowdown (98% speed kept)
//    - 0.0 = instant stop (0% speed kept)
//
// 6. GRAVITY: Constant downward acceleration
//    - Applied only to Y momentum
//    - 1.2 = player falls at realistic speed
//    - 0.0 = no gravity (for backgrounds)
//
// 7. ANIMATIONS: GIF files displayed as sprite sheets
//    - AnimatedSprite loads and manages frames
//    - FPS controls playback speed
//    - SetAnimation changes what animation is playing
//
// 8. EVENTS: Trigger actions based on input
//    - GameEvent::KeyPress: When key goes down
//    - GameEvent::KeyRelease: When key goes up
//    - Each event specifies an Action to perform
//
// 9. SCROLLING ILLUSION: Player stays still, background moves
//    - Three backgrounds create seamless infinite scrolling
//    - When bg1 scrolls off left, it wraps to the right
//    - Negative momentum on backgrounds = player appears to move right
//
// 10. PLATFORMS: Objects with collision detection
//     - .as_platform() makes objects solid
//     - Player lands on platforms and can't fall through
//     - Ground uses "platform" tag for collision system
//
// ========================================================================