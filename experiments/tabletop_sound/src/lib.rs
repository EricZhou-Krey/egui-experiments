pub mod logic_sheet;
pub mod raytrace;
pub mod scene;
pub mod scene_object;
pub mod state;
pub mod style;
pub mod style_sheet;
pub mod tab;
pub mod tabletop_sound;
pub mod tabs;

/*

- Decouple stuff from mapview ui too much is logic based
- Decouple stuff from Soundview in preparation for later
- Polygon convex hull needs better defintion for ui drawing, there are overlaps that cause visual artefacts
* - For shape finding, implment quadtree, including later for ray collisions

Consider each of the states of each tab and create them approriately,
need to create:
    Tab::MapView,
    Tab::NodeDetails,
    Tab::Console,
    Tab::NodeTree,
    Tab::SoundView,
    Tab::PlayControls,

* - Add macros to make rects, sqaures, and approximate circles, triangles, etc

-> 1 - Define receiver and be able move the reveiver around to catpure ambient sound via linear distance or beizer

For each receiver call collect sound, and play when play controls play is selected,
    - use lifespan ~ distance travelled for each ray to collect information rather than n_bonuces
    - first collect ambient background and shapes with constant sounds and play from all directions
    - second ray cast and calculate the n most influential rays (shortest ray to each emitter)
        - maybe ray cast with lower fidelity for each emitter to find specifically what each emitter is located or sounds like its from
        - for each ray, on collision, n_bounces, transmited ray (with rarefactive index), maybe even curvature calculation/simulation, reflected rays
        - rewatch video to figure out all possibilities, also brainstorm them yourself first

2 - Define ray tracer for sound (edit above if necessary)
    - Define emitter and parameters
    - Define ray and parameters
    - Figure out if you can construct sounds using base 3 wavelengths much like colours,
        are there metamers for sounds?
3 - Workout directional sounds,

- WGPU ray calculations
*/
