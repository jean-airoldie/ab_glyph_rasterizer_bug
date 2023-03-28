# Issue

Under specific circumstances, fonts are not rasterized properly and it results in
smudged text. This only occurs for certain characters using certains fonts
at a specific size. In this repo I have included instances of this bug
for the ubuntu regular font scaled to 12 points.

In the `fonts/` directory, I have included the raw instructed passed to the
rasterizer (after scaling + offset) since its simpler. These instructions
are then deserialized, passed to the rasterizer, then serialized as
RGBA pixels and finally written as a png file to disk. The output
of the instructions can be found at the `glyphs/` directory.

The naming structure is the following: `<font_name>_<character>_<size_in_points>.txt`.

In this example, the `m` and `t` characters are defective while
the `o` and `n` are not. 

Note that the rasterizer is resized to be wider than necessary
to make the smudging effect more obvious, since it extends to the
entirely of the rasterizer's width.
