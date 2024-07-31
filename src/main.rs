use clap::Parser;
use colorgrad;
use lofar_h5parm_rs;
use ndarray::{arr2, s, Array, Array2};
use plotters::prelude::*;
use slint;

slint::include_modules!();

/// A Rust interface to summarise LOFAR H5parm calibration tables.
#[derive(Parser, Debug)]
#[command(name = "LOFAR-H5plot")]
#[command(author = "Frits Sweijen")]
#[command(version = "0.0.0")]
#[command(
    help_template = "{name} \nVersion: {version} \nAuthor: {author}\n{about-section} \n {usage-heading} {usage} \n {all-args} {tab}"
)]
// #[clap(author="Author Name", version, about="")]
struct Args {
    /// H5parm to summarise.
    h5parm: String,
}

fn wrap_phase(p: f64) -> f64 {
    //println!("Before remainder: {}", p);
    //dbg!(p);
    //dbg!(p + std::f64::consts::PI);
    //dbg!((p + std::f64::consts::PI) % (2.0 * std::f64::consts::PI));
    //dbg!((p + std::f64::consts::PI) % (2.0 * std::f64::consts::PI) - std::f64::consts::PI);
    //((a % b) + b) % b
    let wrapped = (p + std::f64::consts::PI).rem_euclid(2.0 * std::f64::consts::PI) - std::f64::consts::PI;
    //println!("After remainder: {}", wrapped);
    wrapped
}

fn normalise_phase(p: f64) -> f64{
    let positive = p + std::f64::consts::PI;
    let min = 0.0;//-std::f64::consts::PI;
    (positive - min) / (2.0 * std::f64::consts::PI)
}

fn render_plot(
    idx_ant: i32,
    h5parm: slint::SharedString,
    solset: slint::SharedString,
    soltab: slint::SharedString,
    width: i32,
    height: i32,
) -> slint::Image {
    let aspect = width as f64 / height as f64;
    println!("Aspect ratio is {}", aspect);
    println!("Width is {}", width);
    println!("Height is {}", height);
    let mut pixel_buffer = slint::SharedPixelBuffer::new(width as u32, (width as f64 / aspect) as u32);
    println!("Inside render function");
    println!("= Plotting antenna {}", idx_ant);
    println!("Opening {}", h5parm.to_string());
    let h5 =
        lofar_h5parm_rs::H5parm::open(&h5parm.to_string(), false).expect("Failed to read h5parm");
    let ss = &h5.get_solset(solset.to_string()).unwrap();
    let st = &ss.get_soltab(soltab.to_string()).unwrap();
    let data = st.get_values();
    let data_ref = data.slice(s![.., .., -1, 0]);
    let data_ant = data.slice(s![.., .., idx_ant as usize, 0]);
    let naxis1 = data_ant.shape()[0];
    let naxis2 = data_ant.shape()[1];

    // Construct the plot
    let size = (pixel_buffer.width(), pixel_buffer.height());
    let backend = BitMapBackend::with_buffer(pixel_buffer.make_mut_bytes(), size);
    let root = backend.into_drawing_area();
    root.fill(&plotters::prelude::WHITE)
        .expect("RENDER: Failed to draw to drawing area");

    let mut chart = ChartBuilder::on(&root)
        .caption("Plot!", ("sans-serif", 24))
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0..naxis1 as i32, naxis2 as i32..0)
        .expect("RENDER: error building coordinates");

    chart
        .configure_mesh()
        .x_labels(15)
        .y_labels(15)
        .disable_x_mesh()
        .disable_y_mesh()
        .label_style(("sans-serif", 20))
        .draw()
        .expect("RENDER: error drawing");

    //let ch: cube_helix::CubeHelix = Default::default();
    let color = colorgrad::sinebow();

    chart
        .draw_series(
            (0i32..naxis1 as i32)
                .flat_map(move |i| {
                    (0i32..naxis2 as i32).map(move |j| (i, j, data_ant[[i as usize, j as usize]] - data_ref[[i as usize, j as usize]]))
                })
                .map(|(i, j, d)| {
                    //let color = ch.get_color(normalise_phase(wrap_phase(d)));
                    //dbg!(color);
                    let c = color.at(normalise_phase(wrap_phase(d))).to_linear_rgba_u8();
                    //dbg!(c)
                    Rectangle::new(
                        [(i, naxis2 as i32 - j), (i + 1, naxis2 as i32 - j + 1)],
                        RGBColor(c.0, c.1, c.2)
                        //HSLColor(
                        //    240.0 / 360.0 - 240.0 / 360.0 * d / std::f64::consts::PI as f64,
                        //    1.0,
                        //    0.5,
                        //)
                        .filled(),
                    )
                }),
        )
        .expect("RENDER: failed to render plot");
    drop(chart);
    drop(root);
    slint::Image::from_rgb8(pixel_buffer)
}

fn main() -> Result<(), slint::PlatformError> {
    let args = Args::parse();
    let h5name = args.h5parm;
    let h5 = lofar_h5parm_rs::H5parm::open(&h5name, false).expect("Failed to read h5parm");
    let ss = &h5.solsets[0];
    //let st = &ss.soltabs[0];
    let st = &ss.get_soltab("phase000".to_string()).unwrap();
    let ants = st.get_antennas();

    let ss_names: Vec<slint::SharedString> = h5
        .get_solset_names()
        .into_iter()
        .map(|x| slint::SharedString::from(x.as_str()))
        .collect();
    let sss_model = std::rc::Rc::new(slint::VecModel::from(ss_names.clone()));

    let st_names: Vec<slint::SharedString> = ss
        .get_soltab_names()
        .into_iter()
        .map(|x| slint::SharedString::from(x.as_str()))
        .collect();
    let sts_model = std::rc::Rc::new(slint::VecModel::from(st_names.clone()));

    let dirs: Vec<slint::SharedString> = st
        .get_directions()
        .clone()
        .into_iter()
        .map(|x| slint::SharedString::from(x.as_str()))
        .collect();
    let dirs_model = std::rc::Rc::new(slint::VecModel::from(dirs));

    let stations: Vec<slint::StandardListViewItem> = ants
        .clone()
        .into_iter()
        .map(|x| slint::StandardListViewItem::from(x.as_str()))
        .collect();
    let stations_model = std::rc::Rc::new(slint::VecModel::from(stations));

    let refants: Vec<slint::SharedString> = ants
        .clone()
        .into_iter()
        .map(|x| slint::SharedString::from(x.as_str()))
        .collect();
    let refant_model = std::rc::Rc::new(slint::VecModel::from(refants));

    let ui = AppWindow::new()?;

    ui.set_solset_list(sss_model.into());
    ui.set_soltab_list(sts_model.into());
    ui.set_dir_list(dirs_model.into());
    ui.set_station_list(stations_model.into());
    ui.set_refant_list(refant_model.into());
    ui.set_current_antenna(slint::StandardListViewItem::from(ants[0].as_str()));

    ui.set_h5parm(h5name.clone().into());
    ui.set_solset(ss_names[0].clone().into());
    ui.set_soltab(st_names[0].clone().into());

    ui.on_plot({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let antname = ui.get_current_antenna().text;
            println!("Plotting {}", antname);
            let ui2 = PlotWindow::new().expect("Failed to create plot window.");
            ui2.set_window_title(antname);
            ui2.set_idx_ant(ui.get_current_antenna_idx());
            ui2.set_h5parm(h5name.clone().into());
            ui2.set_solset(ui.get_solset());
            ui2.set_soltab(ui.get_soltab());

            ui2.on_render_plot(render_plot);
            let _ = ui2.run();
        }
    });

    ui.run()
}
