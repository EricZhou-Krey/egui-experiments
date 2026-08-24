pub mod logic_sheet;
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
     -> Tab::MapView,
    Tab::NodeDetails,
    Tab::Console,
    Tab::NodeTree,
    Tab::SoundView,
    Tab::PlayControls,

* - Add macros to make rects, sqaures, and approximate circles, triangles, etc
* - Make it look a bit nice

1 - Define receiver and be able move the reveiver around to catpure ambient sound via linear distance or beizer

2 - Define ray tracer for sound (edit above if necessary)
    - Define emitter and parameters
    - Define ray and parameters
    - Figure out if you can construct sounds using base 3 wavelengths much like colours,
        are there metamers for sounds?
3 - Workout directional sounds,

- WGPU ray calculations


* - For walls specifically, implment quadtree for optimized ray collision search
* - Decouple mapview tools usage
* - Naive find used, should use quadtrees later
*/
