use cached::proc_macro::cached;
use clap::Parser;
use colorgrad;
use lofar_h5parm_rs;
use ndarray::{s, ArrayBase, Dim, OwnedRepr};
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
    let wrapped = (p + std::f64::consts::PI).rem_euclid(2.0 * std::f64::consts::PI) - std::f64::consts::PI;
    wrapped
}

fn normalise_phase(p: f64) -> f64{
    let positive = p + std::f64::consts::PI;
    let min = 0.0;
    (positive - min) / (2.0 * std::f64::consts::PI)
}

#[cached]
fn get_data(h5parm: String, solset: String, soltab: String, idx_ant: i32) -> ArrayBase<OwnedRepr<f64>, Dim<[usize; 2]>>{
    let h5 =
        lofar_h5parm_rs::H5parm::open(&h5parm, false).expect("Failed to read h5parm");
    let ss = h5.get_solset(solset).unwrap();
    let st = ss.get_soltab(soltab).unwrap();
    let data =st.get_values();
    let data_ref = data.slice(s![.., .., idx_ant, 0]);
    data_ref.to_owned()
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
    let mut pixel_buffer = slint::SharedPixelBuffer::new(width as u32, (width as f64 / aspect) as u32);

    // TODO: replace this with a data buffer to avoid reading the file every time
    //println!("Loading data from h5parm");
    let data_ref = get_data(h5parm.to_string(), solset.to_string(), soltab.to_string(), -1);
    let data_ant = get_data(h5parm.to_string(), solset.to_string(), soltab.to_string(), idx_ant);
    let naxis1 = data_ant.shape()[0] as usize;
    let naxis2 = data_ant.shape()[1] as usize;

    // Construct the plot
    //println!("Constructing plot");
    let size = (pixel_buffer.width(), pixel_buffer.height());
    let backend = BitMapBackend::with_buffer(pixel_buffer.make_mut_bytes(), size);
    let root = backend.into_drawing_area();
    root.fill(&plotters::prelude::WHITE)
        .expect("RENDER: Failed to draw to drawing area");

    //println!("== Creating chart");
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

    //println!("== Drawing pixels");
    chart
        .draw_series(
            (0..naxis1)
                .flat_map(move |i| {
                    (0..naxis2).map(move |j| (i, j))
                })
                .map(|(i, j)| {
                    //let color = ch.get_color(normalise_phase(wrap_phase(d)));
                    let d = data_ant[[i, j]] - data_ref[[i, j]];
                    let c = color.at(normalise_phase(wrap_phase(d))).to_linear_rgba_u8();
                    Rectangle::new(
                        [(i as i32, (naxis2 - j) as i32), ((i + 1) as i32, (naxis2 - j + 1) as i32)],
                        RGBColor(c.0, c.1, c.2)
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

    // TODO: cache should be created here

    ui.on_plot({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let antname = ui.get_current_antenna().text;
            println!("Plotting {}", antname);
            let ui2 = PlotWindow2D::new().expect("Failed to create plot window.");
            ui2.set_window_title(antname);
            ui2.set_idx_ant(ui.get_current_antenna_idx());
            ui2.set_h5parm(h5name.clone().into());
            ui2.set_solset(ui.get_solset());
            ui2.set_soltab(ui.get_soltab());

            // TODO: the render function should somehow have access to the cache
            // to avoid loading data every time
            ui2.on_render_plot(render_plot);
            let _ = ui2.run();
        }
    });

    ui.run()
}
