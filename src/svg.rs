
use stones::boards::Lae;
use stones::boards::lae_from_spec;
use stones::layout::Layout;
use stones::layout::LayoutTrait;
use crate::Face::*;
use indoc::*;
use clap::Parser;

// Command-line arguments.

#[derive(Parser)]
struct CLI {
    #[arg()]                                     spec:   String,
    #[arg(short, long, default_value_t = 0.0)]   rotate: f32,
    #[arg(       long, default_value_t = false)] blank:  bool,
    #[arg(short, long)]                          back:   Option<String>,
}

// Face enum.

#[derive(PartialEq)]
enum Face {
    Front,
    Blank,
    Back(String),
}

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

fn svg_from_lae(lae: Lae, rotation: f32, face: Face) -> String {
    let dpi = 100.0; // 300.0; // pixels per inch
    let stone_diam_in = 0.875; // stone diameter in inches
    let line_width_in = 0.03;  // width of each edge line in inches
    let wood_extra_in = 0.15;  // 0.00;  // shortest distance between a stone's edge and the board's edge
    let img_margin_in = 0.5;   // 0.0;   // shortest distance between the board's edge and the image's edge
    let board_color = "#DCB35C"; //"#FFFFFF";
    let line_color  = "#000"; // "#DDD";

    // Private computations.

    let (mut layout, edges) = lae;
    layout = normalize(layout);
    layout = layout.scale(stone_diam_in);
    layout = layout.scale(dpi);
    layout = layout.rotate(rotation);
    if matches!(face, Back(_)) {layout = layout.mirror();}

    let distance = dpi * (stone_diam_in / 2.0 + wood_extra_in + img_margin_in);

    let left   = layout.iter().map(|&n| n.0).reduce(f32::min).unwrap() - distance;
    let right  = layout.iter().map(|&n| n.0).reduce(f32::max).unwrap() + distance;
    let top    = layout.iter().map(|&n| n.1).reduce(f32::min).unwrap() - distance;
    let bottom = layout.iter().map(|&n| n.1).reduce(f32::max).unwrap() + distance;

    let width  = right - left;
    let height = bottom - top;

    let line_width    = dpi * line_width_in;
    let bg_line_width = dpi * (stone_diam_in + wood_extra_in * 2.0);

    let lopen  = if face == Front {""} else {"!--"};
    let lclose = if face == Front {""} else {"--"};

    // Stroke pattern.
    
    let mut strokes = "".to_string();

    for edge in edges {
        strokes.push_str(&format!("M {} {} ", layout[edge.0].0, layout[edge.0].1));
        strokes.push_str(&format!("L {} {} ", layout[edge.1].0, layout[edge.1].1));
    }

    // SVG data.

    let font_size = 36;
    let line_sep = 48;
    let mid_line = 13;

    let svg_text_lines = if let Back(text) = face {
        let dy_start = mid_line - (line_sep/2) * text.matches("|").count() as isize;
        text.split("|")
            .enumerate()
            .map(|(id, line)| format!(r#"<text dy="{}" class="text">{}</text>"#,
                                      dy_start + (line_sep as isize * id as isize), line))
            .collect::<Vec<_>>()
            .join("\n")
    } else {
        "".to_string()
    };

    return formatdoc!(r##"
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="{left} {top} {width} {height}">
            <!--polygon fill="{board_color}" points="500,0 249.99998,433.01273 -250.00003,433.0127 -500,-0.00004371139 -249.99995,-433.01273 250.00018,-433.0126 "/-->
            <path stroke="{board_color}" stroke-linecap="round" stroke-width="{bg_line_width}" fill="none" d="{strokes}" />
            <{lopen}path stroke="{line_color}" stroke-linecap="round" stroke-width="{line_width}" fill="none" d="{strokes}" /{lclose}>

            <style>
                .text {{
                    font-size: {font_size};
                    text-anchor: middle;
                    font-family: lora;
                }}
            </style>
            {svg_text_lines}
        </svg>
    "##);
}

fn main() {
    let args = CLI::parse();

    if args.blank && args.back.is_some() {
        eprintln!("Error: cannot use --blank and --back together.");
        std::process::exit(-1);
    }

    let lae = lae_from_spec(&args.spec).unwrap();
    let face = if args.blank {
        Blank
    } else if args.back.is_none() {
        Front
    } else {
        Back(args.back.unwrap())
    };

    eprintln!("Point count: {}", lae.0.len());
    println!("{}", svg_from_lae(lae, args.rotate, face))
}

