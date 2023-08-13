
use stones::boards::Lae;
use stones::boards::lae_from_spec;
use stones::engine::{Board, Position, Color::*};
use stones::layout::Layout;
use stones::layout::LayoutTrait;
use indoc::*;
use std::env::args;

// "Normalize" a layout by enforcing:
//     1. The shortest distance between two points is 1.
//     2. The center is at (0, 0).

fn normalize(layout: Layout) -> Layout {
    let (left, right, top, bottom) = layout.bounds();
    let min_dist = layout.min_point_separation();

    layout.shift(-(left + right) / 2.0, -(top + bottom)/2.0)
          .scale(1.0 / min_dist)
}

// Produce the SVG text representation of a given board.

fn svg_from_lae(lae: Lae, text: Option<String>, rotation: f32) -> String {
    let dpi = 100.0;           // pixels per inch
    let stone_diam_in = 0.875; // stone diameter in inches
    let line_width_in = 0.03;  // width of each edge line in inches
    let wood_extra_in = 0.15;  // shortest distance between a stone's edge and the board's edge
    let img_margin_in = 0.0;   // shortest distance between the board's edge and the image's edge

    // Private computations.

    let (mut layout, edges) = lae;
    layout = normalize(layout);
    layout = layout.scale(stone_diam_in);
    layout = layout.scale(dpi);
    if text.is_some() {layout = layout.mirror();}
    layout = layout.rotate(rotation);

    let distance = dpi * (stone_diam_in / 2.0 + wood_extra_in + img_margin_in);

    let left   = layout.iter().map(|&n| n.0).reduce(f32::min).unwrap() - distance;
    let right  = layout.iter().map(|&n| n.0).reduce(f32::max).unwrap() + distance;
    let top    = layout.iter().map(|&n| n.1).reduce(f32::min).unwrap() - distance;
    let bottom = layout.iter().map(|&n| n.1).reduce(f32::max).unwrap() + distance;

    let width  = right - left;
    let height = bottom - top;

    let line_width    = dpi * line_width_in;
    let bg_line_width = dpi * (stone_diam_in + wood_extra_in * 2.0);

    let lopen  = if text.is_none() {""} else {"!--"};
    let lclose = if text.is_none() {""} else {"--"};

    // Stroke pattern.
    
    let mut strokes = "".to_string();

    for edge in edges {
        strokes.push_str(&format!("M {} {} ", layout[edge.0].0, layout[edge.0].1));
        strokes.push_str(&format!("L {} {} ", layout[edge.1].0, layout[edge.1].1));
    }

    // SVG data.

    let svg_text_lines = if let Some(text) = text {
        let dy_start = 10 - 20 * text.matches("|").count() as isize;
        text.split("|")
            .enumerate()
            .map(|(id, line)| format!(r#"<text dy="{}" class="text">{}</text>"#,
                                      dy_start + (40 * id) as isize, line))
            .collect::<Vec<_>>()
            .join("\n")
    } else {
        "".to_string()
    };

    return formatdoc!(r##"
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="{left} {top} {width} {height}">
            <path stroke="#DCB35C" stroke-linecap="round" stroke-width="{bg_line_width}" fill="none" d="{strokes}" />
            <{lopen}path stroke="#000" stroke-linecap="round" stroke-width="{line_width}" fill="none" d="{strokes}" /{lclose}>

            <style>
                .text {{
                    font-size: 30;
                    text-anchor: middle;
                    font-family: lora;
                }}
            </style>
            {svg_text_lines}
        </svg>
    "##);

    //for i in 0..layout.len() {
        //if position[i] != Empty {
            //ret.push_str(&format!("<circle fill=\"{}\" cx=\"{}\" cy=\"{}\" r=\"{}\"/>\n",
                                  //if position[i] == Black {"#000"} else {"#FFF"},
                                  //layout[i].0 * dpi, layout[i].1 * dpi, dpi * stone_diam_in / 2.0));
        //}
    //}
}

//fn svg_from_lae_blank(lae: Lae) -> String {
    //let board = Board::new(lae.1.clone());
    //svg_from_lae(lae, board.empty_position())
//}

fn main() {
    let mut args = args(); args.next();
    let spec = args.next().unwrap();
    let text = args.next();
    let angle: f32 = args.next().unwrap_or("0".to_string()).parse().unwrap();

    let lae = lae_from_spec(&spec).unwrap();

    eprintln!("Point count: {}", lae.0.len());
    println!("{}", svg_from_lae(lae, text, angle));
}

