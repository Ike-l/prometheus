use crate::prelude::*;

pub mod prelude {}

pub struct ScalePlugin;

impl PluginTrait for ScalePlugin {
    fn build(&self, _app: &mut App) {
        //todo!()
    }

    fn id(&self) -> PluginId {
        PluginId("prometheus_ScalePlugin")    
    }
}

/*
Syntax rules: 
Using `[]` means you should read the whole sentence, each time, replacing the token with one of the elements.
(or refers to a heading)
Using `(,)` means both of the elements at the same time.
Using `()` means the inner is a separate and collected token.
Using `\` as the escaping character
Using \`\` clarifies a variable or name is involved

GOAL OVERVIEW: 
* Want to set an Object's attributes based on another's
* Want to set an Object's position based on another's

* Result: An object which is "Contained" in another

DECOMPOSITION:
get Req (1): Dimensions (height,width). 
get Req (2): Wall vectors, i.e left wall.
get Req (3): Wall vertices, i.e bottom left position. 

set Req (1): Dimensions (height,width) via `ModelMatrix`
set Req (2): Positions min via `ModelMatrix`
set Req (3): Positions (min,max) Use Req (2) & Req (1)

SOLUTION:
# Rotation means it would get the wrong dimensions
get (1): apply ((model matrix) - rotation) to (vectors between [x,y] from ((model aabb) (min,max))). Magnitudes are `(width,height)`.
get (2): apply (model matrix [| If only used rotation then a negative scale would cause the directions to be flipped?]) to (vectors between [x,y] from ((model aabb) (min,max))). Directions are `Wall vectors`.
get (3): apply (model matrix) to (model aabb). New aabb has the `Wall vertices`.

set (1): set (model scale) to target / existing
set (2): ```
* Copy over both rotations
* Reset model translation
* Find inner model matrix as m1
* Find outer model matrix from target as m2
* Apply inverse of m2 then inverse of m1 to target min as new min
* Find the difference between new min and existing min as model translation
* Find model matrix as m3
* Apply inverse of m3 to target min as new min
* Find the difference between new min and existing min as world translation
```
set (3): set min, then set the width and height along a wall

EXCEPTION CASES:
`container` as the `window`:
get Req (1): `Res<WindowDimensions> from `RenderPlugin`` 
get Req (2): Always(?) `[0, 1, 0]` for `left wall`, `[1, 0, 0]` for `bottom wall`
get Req (3): use a "camera" and "WGPU NDC"

APPLICATION:
-- Example: Object1 as `o1` and `container`, Object2 as `o2` and `contained`, [x,y,a,b] as floats;
# * [set Req (2) | set Req (3)] o2.[min,max].[x,y] to [get Req (3)] o1.[min,max].[x,y] ([+,-] [get Req (1)] o1.[width,height] (*[x,y,a,b]) along [+,-] [get Req (2)]
* set o2.min.x to o1.min.x (+ o1.width (*x) along (bottom wall)).
* set o2.min.y to o1.min.y (+ o1.height (*a) along (left wall)). 
* set o2.max.x to o1.max.x (- o1.width (*y) along -(bottom wall)). 
* set o2.max.y to o1.max.y (- o1.height (*b) along -(left wall)).

Note: using `set min and max` strategy. Can use simple algebra to determine the `set min and dimensions` strategy - Which uses [set (1)]
Note: NOT using "adds", always "set" since i expect the function to be called every tick and people would want a moving shape
*/



