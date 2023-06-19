
fn hex_svg() {
    let layout = layout_trihex(5);
    let edges = edges_trihex(5);

    let scale = 100.0; // "pixels" per inch
    let len = 0.875 * scale;
    //let margin = 0.5 * scale;
    let window = 5.5 * scale;
    let width = 0.03125 * scale;

    let x = layout[45].0;
    let y = layout[45].1;

    println!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
    println!("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"960\" height=\"960\" viewBox=\"-{} -{} {} {}\">",
             window, window, window * 2.0, window * 2.0);
    print!("<polygon fill=\"#DCB35C\" points=\"");

    for i in 0..6 {
        //let radius = len * 5.0 + margin;
        let radius = 5.0 * scale;
        let x = ((i as f32) * TAU / 6.0).cos() * radius;
        let y = ((i as f32) * TAU / 6.0).sin() * radius;
        print!("{},{} ", x, y)
    }

    println!("\"/>");
    print!("<path stroke=\"#000\" stroke-width=\"{}\" fill=\"none\" d=\"", width);

    for edge in edges {
        print!("M {} {} ", (layout[edge.0].0 - x) * len, (layout[edge.0].1 - y) * len);
        print!("L {} {} ", (layout[edge.1].0 - x) * len, (layout[edge.1].1 - y) * len);
    }

    println!("\"/>");
    println!("</svg>");
}

