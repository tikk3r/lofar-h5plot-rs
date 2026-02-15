use h5o3;
use ndarray::s;

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

    match st.get_type().to_lowercase().as_str() {
        "phase" => {
            let phases_wrapped = values
                .slice(s![.., 0, index, 0, 0])
                .iter()
                .zip(values.slice(s![.., 0, index_ref, 0, 0]))
                .map(|(x, x_ref)| {
                    ((x - x_ref) + std::f64::consts::PI).rem_euclid(2.0 * std::f64::consts::PI)
                        - std::f64::consts::PI
                })
                .collect::<Vec<f64>>();
            phases_wrapped
        }
        _ => values.slice(s![.., 0, index, 0, 0]).to_vec(),
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
    dbg!(st.get_type());

    match st.get_type().to_lowercase().as_str() {
        "phase" => {
            let phases_wrapped = values
                .slice(s![0, .., index, 0, 0])
                .iter()
                .zip(values.slice(s![0, .., index_ref, 0, 0]))
                .map(|(x, x_ref)| {
                    ((x - x_ref) + std::f64::consts::PI).rem_euclid(2.0 * std::f64::consts::PI)
                        - std::f64::consts::PI
                })
                .collect::<Vec<f64>>();
            phases_wrapped
        }
        _ => values.slice(s![0, .., index, 0, 0]).to_vec(),
    }
}

//[src/lib.rs:48:5] &phases.shape() = [
//    225,
//    120,
//    58,
//    1,
//    2,
//]

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
            get_values_frequency
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
