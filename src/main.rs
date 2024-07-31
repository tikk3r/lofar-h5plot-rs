use clap::Parser;
use lofar_h5parm_rs;
use ndarray::{s, Array2};
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

fn render_plot(idx_ant: i32) -> slint::Image {
    let mut pixel_buffer = slint::SharedPixelBuffer::new(640, 480);
    println!("Inside render function");
    println!("= Plotting antenna {}", idx_ant);
    // todo load data from h5parm
    let size = (pixel_buffer.width(), pixel_buffer.height());
    let backend = BitMapBackend::with_buffer(pixel_buffer.make_mut_bytes(), size);
    let root = backend.into_drawing_area();
    root.fill(&plotters::prelude::WHITE).expect("RENDER: Failed to draw to drawing area");

    drop(root);
    slint::Image::from_rgb8(pixel_buffer)
}

fn main() -> Result<(), slint::PlatformError> {
    let args = Args::parse();
    let h5name = args.h5parm;
    let h5 = lofar_h5parm_rs::H5parm::open(&h5name, false).expect("Failed to read h5parm");
    let ss = &h5.solsets[0];
    let st = &ss.soltabs[0];
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
    let sts_model = std::rc::Rc::new(slint::VecModel::from(st_names));

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

    let data = st.get_values();
    dbg!(data.shape());
    //dbg!(data.slice(s![.., .., 0, 0, 0]));

    let ui = AppWindow::new()?;

    ui.set_solset_list(sss_model.into());
    ui.set_soltab_list(sts_model.into());
    ui.set_dir_list(dirs_model.into());
    ui.set_station_list(stations_model.into());
    ui.set_refant_list(refant_model.into());
    ui.set_current_antenna(slint::StandardListViewItem::from(ants[0].as_str()));

    ui.on_plot({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let antname = ui.get_current_antenna().text;
            println!("Plotting {}", antname);
            let ui2 = PlotWindow::new().expect("Failed to create plot window.");
            ui2.set_window_title(antname);
            ui2.set_idx_ant(ui.get_current_antenna_idx());

            ui2.on_render_plot(render_plot);
            let _ = ui2.run();
        }
    });

    ui.run()
}
