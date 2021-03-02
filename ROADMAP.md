# ROADMAP
This document describes the features needed for future releases.

Past Releases
-------------
### (v0.1.0) - December 2020
First version of tuix released and demonstrated to the RustAudio community. Subsequent development has focused on usability and integration with tools to allow for the creation of audio plugins.


Future Work (v0.2.0)
--------------------
This section details the planned features required for a (v0.2.0) release. The target release date is mid-late February.

### Documentation & Tutorials
 * [ ] **Website** - A website (github pages) is in development to document the features and usage of tuix.

### Built-in widgets
  * [ ] **Scroll Container** - Fix horizontal and vertical scroll containers.
  
### Events
  * [ ] **Window close** - Requires access to the window handle provided by winit or baseview.
  * [ ] **Set cursor icon** - Cursor icons are set by the window handle which is not currently accessible by the events system.
  
### Styling
  * [ ] **Background gradients**
  * [ ] **Background images**
  * [ ] **Individual borders** - Add ability to set top, bottom, left, and right borders. This is tricky because femtovg has no built in support. It can be done manually but only for boxes with no rounded corners.
  * [ ] **Animation Direction** - Add ability to specify a reverse direction for animations.
  * [ ] **Animation Fill Mode** - Add support to specify the style properties an entity has before and after an animation.
  * [ ] **Animation Timing Function** - Add support for different animation timing functions.
  * [ ] **Animation Iteration Count** - Add support for multiple iterations of the same animation.
  * [ ] **Multiple Animation Keyframes** - Currently animations only support a start and end keyframe. Add support for multiple keyframes.
 
### Layout
  * [ ] **Integer Lengths** - Integer widths and heights for flexible elements when growing and shrinking.
  
### Rendering
 * [ ] **Convenience functions** - for drawing different parts of widgets - e.g. drawing just the shadow, or just the test, etc.
 
Beyond v0.2.0
-------------
 * [ ] **Multi-window support** - The main part to decide is whether different windows should have their own State or if the State could be shared between multiple windows. I think the latter would be difficult due to borrowing rules and the fact that windows might be in different threads. If each window has its own state then how should they communictae with each other?
 * [ ] **Grid** - Implement a grid layout system
 * [ ] **Better Text** - Better loking text
 * [ ] **Text Layout** - Expose more control over text layout
