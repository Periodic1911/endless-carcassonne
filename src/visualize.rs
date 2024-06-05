use crate::wave_function::TileMap;
use itertools::Itertools;
use std::fs::write;
use std::path::Path;

pub fn visualize<P: AsRef<Path>>(map: TileMap, path: P) {
    let html = (0..map.height() as isize)
        .rev()
        .cartesian_product(0..map.width() as isize)
        .map(|(y, x)| format!("<div class=\"grid-square {}\"></div>", map[(x, y)].unwrap()))
        .join("");

    let style = include_str!("../resources/style.css");

    let output = format!(
        "\
<!doctype HTML>\
    <html>\
        <head>\
            <style>\
                {style}\
                main {{\
                    grid-template-columns: repeat({}, auto);\
                    grid-template-rows: repeat({}, auto);\
                }}\
            </style>\
        </head>\
    <main>\
        {html}\
    </main>\
</html>",
        map.height(),
        map.width()
    );

    write(path, output).unwrap();
}
