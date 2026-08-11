# Project Notes

* **Triangulation Graph:** Successfully implemented.
* **Renderer:** Currently inefficient. Relies on native `painter`, but should be refactored to leverage `eframe`'s access to `wgpu` for better performance.
* **Overlays & Navigation:** Still developing overlays for different experiments. The original overlay navigation system was scrapped; we are now using a similar concept integrated directly into the main project navigator.
