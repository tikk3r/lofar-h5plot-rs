use h5o3;
use ndarray::{Axis, Ix2, Slice};

#[tauri::command]
fn get_h5parm_name() -> String {
    let args: Vec<String> = std::env::args().collect();
    args[1].clone()
}

#[tauri::command]
fn get_station_names(h5: String, solset: String, soltab: String) -> Vec<String> {
    let h5parm = h5o3::H5parm::open(&h5, true).expect("Failed to read H5parm.");
    let ss = h5parm.get_solset(solset).expect("Failed to read solset.");
    let st = ss.get_soltab(soltab).expect("Failed to read soltab.");
    let stations = st.get_antennas();
    stations.iter().map(|x| x.to_string()).collect()
}

#[tauri::command]
fn get_solset_names(h5: String) -> Vec<String> {
    let h5parm = h5o3::H5parm::open(&h5, true).expect("Failed to read H5parm.");
    h5parm.get_solset_names().clone()
}

#[tauri::command]
fn get_soltab_names(h5: String, solset: String) -> Vec<String> {
    let h5parm = h5o3::H5parm::open(&h5, true).expect("Failed to read H5parm.");
    let ss = &h5parm.get_solset(solset).expect("Failed to load solset");
    ss.get_soltab_names()
}

#[tauri::command]
fn get_soltab_times(h5: String, solset: String, soltab: String) -> Vec<f64> {
    let h5parm = h5o3::H5parm::open(&h5, true).expect("Failed to read H5parm.");
    let ss = &h5parm.get_solset(solset).expect("Failed to load solset");
    let st = ss.get_soltab(soltab).expect("Failed to read soltab.");
    st.get_times().to_vec()
}

#[tauri::command]
fn get_soltab_freqs(h5: String, solset: String, soltab: String) -> Vec<f64> {
    let h5parm = h5o3::H5parm::open(&h5, true).expect("Failed to read H5parm.");
    let ss = &h5parm.get_solset(solset).expect("Failed to load solset");
    let st = ss.get_soltab(soltab).expect("Failed to read soltab.");
    st.get_frequencies().to_vec()
}

#[tauri::command]
fn get_values_time(
    h5: String,
    solset: String,
    soltab: String,
    antenna: String,
    refant: String,
) -> Vec<f64> {
    let h5parm = h5o3::H5parm::open(&h5, true).expect("Failed to read H5parm.");
    let ss = &h5parm.get_solset(solset).expect("Failed to load solset");
    let st = ss.get_soltab(soltab).expect("Failed to read soltab.");
    let values = st.get_values().to_owned();
    let stations = st.get_antennas();
    let index = stations.iter().position(|&x| x == antenna).unwrap();
    let index_ref = stations.iter().position(|&x| x == refant).unwrap();
    let axes = st.get_axes();

    let idx_time = axes.iter().position(|x| x == "time").unwrap();
    let idx_freq = axes.iter().position(|x| x == "freq").unwrap();
    let idx_ant = axes.iter().position(|x| x == "ant").unwrap();

    let idx_dir = axes.iter().position(|x| x == "dir").unwrap_or(99);
    let idx_pol = axes.iter().position(|x| x == "pol").unwrap_or(99);

    let data = if (&axes).len() == 5 {
        values
            .slice_each_axis(|ax| {
                let i = ax.axis.index();
                if i == idx_pol {
                    Slice::from(0..1)
                } else if i == idx_dir {
                    Slice::from(0..1)
                } else {
                    Slice::from(..)
                }
            })
            .to_owned()
    } else if ((&axes).len() == 4) && (idx_dir != 99) {
        values
            .slice_each_axis(|ax| {
                let i = ax.axis.index();
                if i == idx_dir {
                    Slice::from(0..1)
                } else {
                    Slice::from(..)
                }
            })
            .to_owned()
    } else if ((&axes).len() == 4) && (idx_pol != 99) {
        values
            .slice_each_axis(|ax| {
                let i = ax.axis.index();
                if i == idx_pol {
                    Slice::from(0..1)
                } else {
                    Slice::from(..)
                }
            })
            .to_owned()
    } else {
        panic!("Should not arrive here!");
    };

    match st.get_type().to_lowercase().as_str() {
        "phase" => {
            let values = data
                .slice_each_axis(|ax| {
                    let i = ax.axis.index();
                    if i == idx_time {
                        Slice::from(..)
                    } else if i == idx_freq {
                        Slice::from(0..1)
                    } else if i == idx_ant {
                        Slice::from(index..index + 1)
                    } else {
                        Slice::from(..)
                    }
                })
                .to_owned();
            let values_ref = data
                .slice_each_axis(|ax| {
                    let i = ax.axis.index();
                    if i == idx_time {
                        Slice::from(..)
                    } else if i == idx_freq {
                        Slice::from(0..1)
                    } else if i == idx_ant {
                        Slice::from(index_ref..index_ref + 1)
                    } else {
                        Slice::from(..)
                    }
                })
                .to_owned();
            (values - values_ref).into_raw_vec_and_offset().0
        }
        _ => {
            data.slice_each_axis(|ax| {
                let i = ax.axis.index();
                if i == idx_time {
                    Slice::from(..)
                } else if i == idx_freq {
                    Slice::from(0..1)
                } else if i == idx_ant {
                    Slice::from(index..index + 1)
                } else {
                    Slice::from(..)
                }
            })
            .to_owned()
            .into_raw_vec_and_offset()
            .0
        }
    }
}

#[tauri::command]
fn get_values_frequency(
    h5: String,
    solset: String,
    soltab: String,
    antenna: String,
    refant: String,
) -> Vec<f64> {
    let h5parm = h5o3::H5parm::open(&h5, true).expect("Failed to read H5parm.");
    let ss = &h5parm.get_solset(solset).expect("Failed to load solset");
    let st = ss.get_soltab(soltab).expect("Failed to read soltab.");
    let values = st.get_values().to_owned();
    let stations = st.get_antennas();
    let index = stations.iter().position(|&x| x == antenna).unwrap();
    let index_ref = stations.iter().position(|&x| x == refant).unwrap();
    let axes = st.get_axes();

    let idx_time = axes.iter().position(|x| x == "time").unwrap();
    let idx_freq = axes.iter().position(|x| x == "freq").unwrap();
    let idx_ant = axes.iter().position(|x| x == "ant").unwrap();

    let idx_dir = axes.iter().position(|x| x == "dir").unwrap_or(99);
    let idx_pol = axes.iter().position(|x| x == "pol").unwrap_or(99);

    let data = if (&axes).len() == 5 {
        values
            .slice_each_axis(|ax| {
                let i = ax.axis.index();
                if i == idx_pol {
                    Slice::from(0..1)
                } else if i == idx_dir {
                    Slice::from(0..1)
                } else {
                    Slice::from(..)
                }
            })
            .to_owned()
    } else if ((&axes).len() == 4) && (idx_dir != 99) {
        values
            .slice_each_axis(|ax| {
                let i = ax.axis.index();
                if i == idx_dir {
                    Slice::from(0..1)
                } else {
                    Slice::from(..)
                }
            })
            .to_owned()
    } else if ((&axes).len() == 4) && (idx_pol != 99) {
        values
            .slice_each_axis(|ax| {
                let i = ax.axis.index();
                if i == idx_pol {
                    Slice::from(0..1)
                } else {
                    Slice::from(..)
                }
            })
            .to_owned()
    } else {
        panic!("Should not arrive here!");
    };

    match st.get_type().to_lowercase().as_str() {
        "phase" => {
            let values = data
                .slice_each_axis(|ax| {
                    let i = ax.axis.index();
                    if i == idx_time {
                        Slice::from(0..1)
                    } else if i == idx_freq {
                        Slice::from(..)
                    } else if i == idx_ant {
                        Slice::from(index..index + 1)
                    } else {
                        Slice::from(..)
                    }
                })
                .to_owned();
            let values_ref = data
                .slice_each_axis(|ax| {
                    let i = ax.axis.index();
                    if i == idx_time {
                        Slice::from(0..1)
                    } else if i == idx_freq {
                        Slice::from(..)
                    } else if i == idx_ant {
                        Slice::from(index_ref..index_ref + 1)
                    } else {
                        Slice::from(..)
                    }
                })
                .to_owned();
            (values - values_ref).into_raw_vec_and_offset().0
        }
        _ => {
            data.slice_each_axis(|ax| {
                let i = ax.axis.index();
                if i == idx_time {
                    Slice::from(0..1)
                } else if i == idx_freq {
                    Slice::from(..)
                } else if i == idx_ant {
                    Slice::from(index..index + 1)
                } else {
                    Slice::from(..)
                }
            })
            .to_owned()
            .into_raw_vec_and_offset()
            .0
        }
    }
}

#[tauri::command]
fn get_values_waterfall(
    h5: String,
    solset: String,
    soltab: String,
    antenna: String,
    refant: String,
) -> (Vec<f64>, usize, usize) {
    let h5parm = h5o3::H5parm::open(&h5, true).expect("Failed to read H5parm.");
    let ss = &h5parm.get_solset(solset).expect("Failed to load solset");
    let st = ss.get_soltab(soltab).expect("Failed to read soltab.");
    let values = st.get_values().to_owned();
    let stations = st.get_antennas();
    let index = stations.iter().position(|&x| x == antenna).unwrap();
    let index_ref = stations.iter().position(|&x| x == refant).unwrap();
    let axes = st.get_axes();
    //let data = values.clone();
    //let data = data.index_axis(Axis(4), 0);
    //let data = data.index_axis(Axis(3), 0);
    //let data = data.index_axis(Axis(2), index);
    //let mut data: Array2<f64> = data.into_dimensionality::<Ix2>().unwrap().t().to_owned();

    let idx_time = axes.iter().position(|x| x == "time").unwrap();
    let idx_freq = axes.iter().position(|x| x == "freq").unwrap();
    let idx_ant = axes.iter().position(|x| x == "ant").unwrap();

    let idx_dir = axes.iter().position(|x| x == "dir").unwrap_or(99);
    let idx_pol = axes.iter().position(|x| x == "pol").unwrap_or(99);

    let data = if (&axes).len() == 5 {
        values
            .slice_each_axis(|ax| {
                let i = ax.axis.index();
                if i == idx_pol {
                    Slice::from(0..1)
                } else if i == idx_dir {
                    Slice::from(0..1)
                } else {
                    Slice::from(..)
                }
            })
            .to_owned()
    } else if ((&axes).len() == 4) && (idx_dir != 99) {
        values
            .slice_each_axis(|ax| {
                let i = ax.axis.index();
                if i == idx_dir {
                    Slice::from(0..1)
                } else {
                    Slice::from(..)
                }
            })
            .to_owned()
    } else if ((&axes).len() == 4) && (idx_pol != 99) {
        values
            .slice_each_axis(|ax| {
                let i = ax.axis.index();
                if i == idx_pol {
                    Slice::from(0..1)
                } else {
                    Slice::from(..)
                }
            })
            .to_owned()
    } else {
        panic!("Should not arrive here!");
    };

    let data = match st.get_type().to_lowercase().as_str() {
        "phase" => {
            let values = data
                .slice_each_axis(|ax| {
                    let i = ax.axis.index();
                    if i == idx_time {
                        Slice::from(..)
                    } else if i == idx_freq {
                        Slice::from(..)
                    } else if i == idx_ant {
                        Slice::from(index..index + 1)
                    } else {
                        Slice::from(..)
                    }
                })
                .to_owned();
            let values_ref = data
                .slice_each_axis(|ax| {
                    let i = ax.axis.index();
                    if i == idx_time {
                        Slice::from(..)
                    } else if i == idx_freq {
                        Slice::from(..)
                    } else if i == idx_ant {
                        Slice::from(index_ref..index_ref + 1)
                    } else {
                        Slice::from(..)
                    }
                })
                .to_owned();
            let mut values_reffed = values - values_ref;
            let mut i = values_reffed.ndim();
            while i > 2 {
                values_reffed = values_reffed.index_axis(Axis(i - 1), 0).to_owned();
                i = i - 1;
            }

            values_reffed
        }
        _ => {
            let mut values = data
                .slice_each_axis(|ax| {
                    let i = ax.axis.index();
                    if i == idx_time {
                        Slice::from(..)
                    } else if i == idx_freq {
                        Slice::from(..)
                    } else if i == idx_ant {
                        Slice::from(index..index + 1)
                    } else {
                        Slice::from(..)
                    }
                })
                .to_owned();
            let mut i = values.ndim();
            while i > 2 {
                values = values.index_axis(Axis(i - 1), 0).to_owned();
                i = i - 1;
            }
            values
        }
    };
    let data = data.t().to_owned();
    let width = data.shape()[0];
    let height = data.shape()[1];
    (data.as_standard_layout().to_owned().into_raw_vec_and_offset().0, width, height)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_h5parm_name,
            get_station_names,
            get_solset_names,
            get_soltab_names,
            get_soltab_times,
            get_soltab_freqs,
            get_values_time,
            get_values_frequency,
            get_values_waterfall
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
