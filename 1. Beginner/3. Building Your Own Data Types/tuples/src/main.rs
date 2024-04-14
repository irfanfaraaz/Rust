fn main() {
    //tuples
    let rbg_color = (255,106,0);
    let cmyk_color = (0,58,100,0);

    //tuple structs
    struct RgbColor(i32, i32, i32);
    struct CmykColor(i32, i32, i32, i32);

    let rbg_color = RgbColor(255,106,0);
    let cmyk_color = CmykColor(0,58,100,0);

    //unit-like structs

    struct UnitLikeStruct;
}
