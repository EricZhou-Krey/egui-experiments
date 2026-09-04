pub mod ray;

/*

Raytrace Planning
INPUTS
    - Scene Viewer - quick access to collsion points, and normals -should calculate normals dynamically from scene viewer
        - Receiver position to emit rays from, emitters to consume rays or capture sounds

INTERNALS
    - Ray defintion, using lifetimes (~ distance travelled) to determine the contribution level

OUTPUTS
    - Virtual position for each emitter sound, with corresponding filters applied depending
        - Each emitter has a direct, reflected and ambient contribution
            - Direct is located onto of emitters original position and low-pass filter applied if blocked with transmittence contribution
            - Reflected is virtually located at the point of greatest ray contribution from the emitter (loudest)
            - Ambient is collection of residual rays that have little contribution
    - Construct the sound by graphing and moving the 3 virtual emitter corresponding to each emitter where short rays have
    high contribution and far rays come in later to contribute adding a delay for which the sound is simulated

    - Generate a sound environment, and update when apprioate, when things are added or removed and etc
*/
