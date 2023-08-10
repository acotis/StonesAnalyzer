
use stones::boards::Lae;
use stones::boards::Layout;
use stones::boards::lae_from_spec;
use stones::engine::{Board, Position, Color::*};
use indoc::*;

// Linear transform stuff.

fn shift(layout: &Layout, dx: f32, dy: f32) -> Layout {
    layout.iter()
          .map(|&p| (p.0 + dx, p.1 + dy))
          .collect()
}

fn scale(layout: &Layout, scale: f32) -> Layout {
    layout.iter()
          .map(|&p| (p.0 * scale, p.1 * scale))
          .collect()
}

// "Normalize" a layout by enforcing:
//     1. The shortest distance between two points is 1.
//     2. The center is at (0, 0).

fn normalize(layout: &Layout) -> Layout {
    let left   = layout.iter().map(|&n| n.0).reduce(f32::min).unwrap();
    let right  = layout.iter().map(|&n| n.0).reduce(f32::max).unwrap();
    let top    = layout.iter().map(|&n| n.1).reduce(f32::min).unwrap();
    let bottom = layout.iter().map(|&n| n.1).reduce(f32::max).unwrap();

    let mut min_dist: f32 = f32::INFINITY;

    for a in layout {
        for b in layout {
            let dist = f32::hypot(a.0 - b.0, a.1 - b.1);

            if dist > 0.0 && dist < min_dist {
                min_dist = dist;
            }
        }
    }

    scale(&shift(layout, -(left + right) / 2.0, -(top + bottom)/2.0), 1.0 / min_dist)
}

// Produce the SVG text representation of a given board.

fn svg_from_lae(lae: Lae, position: Position) -> String {
    let dpi = 100.0;            // pixels per inch
    let stone_diam_in  = 0.875; // stone diameter in inches
    let line_width_in  = 0.03;  // width of each edge line in inches
    let wood_margin_in = 0.15;  // shortest distance between a stone's edge and the board's edge
    let img_border_in  = 0.1;   // shortest distance between the board's edge and the image's edge

    // Private computations.

    let (mut layout, edges) = lae;
    layout = normalize(&layout);
    layout = scale(&layout, stone_diam_in);
    let distance = stone_diam_in / 2.0 + wood_margin_in + img_border_in;

    let left   = dpi * (layout.iter().map(|&n| n.0).reduce(f32::min).unwrap() - distance);
    let right  = dpi * (layout.iter().map(|&n| n.0).reduce(f32::max).unwrap() + distance);
    let top    = dpi * (layout.iter().map(|&n| n.1).reduce(f32::min).unwrap() - distance);
    let bottom = dpi * (layout.iter().map(|&n| n.1).reduce(f32::max).unwrap() + distance);

    let width  = right - left;
    let height = bottom - top;

    let line_width    = dpi * line_width_in;
    let bg_line_width = dpi * (stone_diam_in + wood_margin_in * 2.0);

    // Stroke pattern.
    
    let mut strokes = "".to_string();

    for edge in edges {
        strokes.push_str(&format!("M {} {} ", layout[edge.0].0 * dpi, layout[edge.0].1 * dpi));
        strokes.push_str(&format!("L {} {} ", layout[edge.1].0 * dpi, layout[edge.1].1 * dpi));
    }

    // SVG data.

    return formatdoc!(r##"
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="{left} {top} {width} {height}">
            <path stroke="#DCB35C" stroke-linecap="round" stroke-width="{bg_line_width}" fill="none" d="{strokes}" />
            <path stroke="#000" stroke-linecap="round" stroke-width="{line_width}" fill="none" d="{strokes}" />

            <style>
                .text {{
                    font-size: 30;
                    text-anchor: middle;
                    font-family: lora;
                }}
            </style>
            <text dy="-30" class="text">The essence of Go</text>
            <text dy="10" class="text">reflected like a moonbeam</text>
            <text dy="50" class="text">in every board shape</text>
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
    let lae = lae_from_spec(&std::env::args().nth(1).unwrap()).unwrap();
    let board = Board::new(lae.1.clone());
    let position = board.empty_position();

    //for i in 0..49 {
        //board.play(&mut position, Black, 2*i);
        //board.play(&mut position, White, 2*i+1);
    //}

    println!("{}", svg_from_lae(lae, position));
    eprintln!("Point count: {}", board.point_count());
}

