pub mod scene;
pub mod state;
pub mod style;
pub mod style_sheet;
pub mod tab;
pub mod tabletop_sound;
pub mod tabs;

/*

TODO: PLAN BETTER
Consider each of the states of each tab and create them approriately,
need to create:
    Tab::MapView,
    Tab::NodeDetails,
    Tab::Console,
    Tab::NodeTree,
    Tab::SoundView,
    Tab::PlayControls,

- Add vertices to scene
- Click position placement
- Draw shapes using polygons
    - Add macros to make rects, sqaures, and approximate circles, triangles, etc
- Attach ambient noise to each node irrelevant or applied to fading out linearly/
or other beizer function defined by user

- Make it look a bit nice

- Define receiver and be able move the reveiver around to catpure ambient sound
- Define ray tracer for sound (edit above if necessary)
    - Define emitter and parameters
    - Define ray and parameters
    - Figure out if you can construct sounds using base 3 wavelengths much like colours,
        are there metamers for sounds?
- WGPU ray calculations

- Workout directional sounds,
*/
