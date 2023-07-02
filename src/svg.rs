
use stones::boards::Lae;
use stones::boards::lae_from_spec;

fn svg_from_lae(lae: Lae) -> String {
    let scale = 300.0;      // "pixels" per inch
    let diam = 0.875;       // stone diameter in inches
    let width = 0.03125;    // width of each edge line in inches
    let margin = 0.5;       // shortest distance between a stone's edge and the board's edge
    let window = 0.5;       // shortest distance between the board's edge and the image's edge

    // Private computations.

    let (layout, edges) = lae;
    let distance = diam / 2.0 + margin + window;

    let left   = scale * (layout.iter().map(|&n| n.0 * diam).reduce(f32::min).unwrap() - distance);
    let right  = scale * (layout.iter().map(|&n| n.0 * diam).reduce(f32::max).unwrap() + distance);
    let top    = scale * (layout.iter().map(|&n| n.1 * diam).reduce(f32::min).unwrap() - distance);
    let bottom = scale * (layout.iter().map(|&n| n.1 * diam).reduce(f32::max).unwrap() + distance);

    // Stroke pattern.
    
    let mut strokes = "".to_string();

    for edge in edges {
        strokes.push_str(&format!("M {} {} ", layout[edge.0].0 * diam * scale, layout[edge.0].1 * diam * scale));
        strokes.push_str(&format!("L {} {} ", layout[edge.1].0 * diam * scale, layout[edge.1].1 * diam * scale));
    }

    // SVG data.

    let mut ret = "".to_string();

    ret.push_str(&format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n"));
    ret.push_str(&format!("<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"{} {} {} {}\">\n",
             left, top, right - left, bottom - top));
    ret.push_str(&format!("<path stroke=\"#DCB35C\" stroke-linecap=\"round\" stroke-width=\"{}\" fill=\"none\" d=\"", scale * (diam / 2.0 + margin)));
    ret.push_str(&strokes);
    ret.push_str("\"/>\n");
    ret.push_str(&format!("<path stroke=\"#000\" stroke-linecap=\"round\" stroke-width=\"{}\" fill=\"none\" d=\"", scale * width));
    ret.push_str(&strokes);
    ret.push_str("\"/>\n");
    ret.push_str("</svg>\n");

    ret
}

fn main() {
    println!("{}", svg_from_lae(lae_from_spec("trihex:5").unwrap()));
}

