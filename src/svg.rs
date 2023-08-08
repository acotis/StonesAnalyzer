
use stones::boards::Lae;
use stones::boards::Layout;
use stones::boards::lae_from_spec;
use stones::engine::{Board, Position, Color::*};

fn svg_from_lae(lae: Lae, position: Position) -> String {
    let scale = 300.0;      // "pixels" per inch
    let diam = 0.875;       // stone diameter in inches
    let width = 0.03;       // width of each edge line in inches
    let margin = 0.15;      // shortest distance between a stone's edge and the board's edge
    let window = 0.1;       // shortest distance between the board's edge and the image's edge

    // Private computations.

    let (layout_stone, edges) = lae;
    let layout: Layout = layout_stone.into_iter().map(|p| (p.0 * diam, p.1 * diam)).collect();
    let distance = diam / 2.0 + margin + window;

    let left   = scale * (layout.iter().map(|&n| n.0).reduce(f32::min).unwrap() - distance);
    let right  = scale * (layout.iter().map(|&n| n.0).reduce(f32::max).unwrap() + distance);
    let top    = scale * (layout.iter().map(|&n| n.1).reduce(f32::min).unwrap() - distance);
    let bottom = scale * (layout.iter().map(|&n| n.1).reduce(f32::max).unwrap() + distance);

    // Stroke pattern.
    
    let mut strokes = "".to_string();

    for edge in edges {
        strokes.push_str(&format!("M {} {} ", layout[edge.0].0 * scale, layout[edge.0].1 * scale));
        strokes.push_str(&format!("L {} {} ", layout[edge.1].0 * scale, layout[edge.1].1 * scale));
    }

    // SVG data.

    let mut ret = "".to_string();

    ret.push_str(&format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n"));
    ret.push_str(&format!("<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"{} {} {} {}\">\n",
             left, top, right - left, bottom - top));
    //ret.push_str("<rect width=\"5000\" height=\"5000\" x=\"-2000\" y=\"-2000\" fill=\"#DCB35C\"/>\n");

    ret.push_str(&format!("<path stroke=\"#DCB35C\" stroke-linecap=\"round\" stroke-width=\"{}\" fill=\"none\" d=\"", scale * (diam + margin * 2.0)));
    ret.push_str(&strokes);
    ret.push_str("\"/>\n");
    ret.push_str(&format!("<path stroke=\"#000\" stroke-linecap=\"round\" stroke-width=\"{}\" fill=\"none\" d=\"", scale * width));
    ret.push_str(&strokes);
    ret.push_str("\"/>\n");

    for i in 0..layout.len() {
        if position[i] != Empty {
            ret.push_str(&format!("<circle fill=\"{}\" cx=\"{}\" cy=\"{}\" r=\"{}\"/>\n",
                                  if position[i] == Black {"#000"} else {"#FFF"},
                                  layout[i].0 * scale, layout[i].1 * scale, scale * diam / 2.0));
        }
    }

    ret.push_str("</svg>\n");

    ret
}

//fn svg_from_lae_blank(lae: Lae) -> String {
    //let board = Board::new(lae.1.clone());
    //svg_from_lae(lae, board.empty_position())
//}

fn main() {
    //let lae = lae_from_spec("turtle:1:1").unwrap();
    //let lae = lae_from_spec("trihex:5").unwrap();
    let lae = lae_from_spec(&std::env::args().nth(1).unwrap()).unwrap();
    let board = Board::new(lae.1.clone());
    let position = board.empty_position();

    //for i in 0..49 {
        //board.play(&mut position, Black, 2*i);
        //board.play(&mut position, White, 2*i+1);
    //}

    println!("{}", svg_from_lae(lae, position));
    //println!("Point count: {}", board.point_count());
}

