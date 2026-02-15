const { invoke } = window.__TAURI__.core;

import * as Plot from '@observablehq/plot';

const plotContainer = document.getElementById('plotting_area');

const renderPlot = function(data, mode) {
      const plot = Plot.plot({
        marks: [Plot.line(data, { x: 'x', y: 'y', stroke: 'blue' }),
            Plot.dot(data, { x: 'x', y: 'y', stroke: 'blue' })],
        width: plotContainer.clientWidth*0.85,
        height: plotContainer.clientHeight*0.85,
        marginBottom: 60,
        marginLeft: 90,
        x: {
            label: ax_selected == "time" ? "Time [s]" : "Frequency [MHz]",
        },
        y: {
          domain: mode == "phase" ? [-Math.PI, Math.PI] : [0, 2],
        },
        style: {
          width: '100%',
          height: '100%',
          maxWidth: 'none',
          maxHeight: 'none',
          objectFit: 'fill',
          fontSize: 24
        },
      });

      plotContainer.innerHTML = '';
      plotContainer.appendChild(plot);
};

window.addEventListener('resize', () => {
    const solset_list = document.getElementById('solset_picker');
    let ss_selected = solset_list.value;
    const soltab_list = document.getElementById('soltab_picker');
    let st_selected = soltab_list.value;
    if (st_selected.includes("phase")) {
        mode = "phase";
    }else if (st_selected.includes("amplitude")) {
        mode = "amplitude";
    }
    renderPlot(data, mode);
});

async function get_h5parm_name() {
    let name = await invoke("get_h5parm_name");
    return name;
}

async function get_station_names(h5parm, ss, st) {
    let names = await invoke("get_station_names", {h5: h5parm, solset: ss, soltab: st});
    return names
}

async function get_solset_names(h5parm) {
    let names = await invoke("get_solset_names", {h5: h5parm});
    return names
}

async function get_soltab_names(h5parm, ss) {
    let names = await invoke("get_soltab_names", {h5: h5parm, solset: ss});
    return names
}

async function get_soltab_times(h5parm, ss, st) {
    let times = await invoke("get_soltab_times", {h5: h5parm, solset: ss, soltab: st});
    return times
}

async function get_soltab_freqs(h5parm, ss, st) {
    let freqs = await invoke("get_soltab_freqs", {h5: h5parm, solset: ss, soltab: st});
    return freqs
}

async function get_values_time(h5parm, ss, st, antenna, refant) {
    let values = await invoke("get_values_time", {h5: h5parm, solset: ss, soltab: st, antenna: antenna, refant: refant});
    return values
}

async function get_values_frequency(h5parm, ss, st, antenna, refant) {
    let values = await invoke("get_values_frequency", {h5: h5parm, solset: ss, soltab: st, antenna: antenna, refant: refant});
    return values
}

let h5parm = "";
let mode = "phase";
let ax_selected = "time";
let data = null;

document.getElementById("button_plot").addEventListener('click', () => {
    console.log("Plotting " + h5parm);
    const solset_list = document.getElementById('solset_picker');
    let ss_selected = solset_list.value;
    const soltab_list = document.getElementById('soltab_picker');
    let st_selected = soltab_list.value;
    const antenna = document.getElementById('antennas').getElementsByClassName("selected")[0].innerHTML;

    const axis_list = document.getElementById('axis_picker');
    ax_selected = axis_list.value;
    if (ax_selected == "time") {
        get_soltab_times(h5parm, ss_selected, st_selected).then(times => {
            console.log(times)
            get_values_time(h5parm, ss_selected, st_selected, antenna, "CS002HBA0").then(values => {
                console.log(values)
                data = times.map((value, index) => ({
                  x: value - times[0],
                  y: values[index],
                }));
                if (st_selected.includes("phase")) {
                    renderPlot(data, "phase");
                }else if (st_selected.includes("amplitude")) {
                    renderPlot(data, "amplitude");
                }
            });
        });
    } else if (ax_selected == "frequency") {
        get_soltab_freqs(h5parm, ss_selected, st_selected).then(freqs => {
            console.log(freqs)
            get_values_frequency(h5parm, ss_selected, st_selected, antenna, "CS002HBA0").then(values => {
                console.log(values)
                data = freqs.map((value, index) => ({
                  x: value / 1e6,
                  y: values[index],
                }));
                if (st_selected.includes("phase")) {
                    renderPlot(data, "phase");
                }else if (st_selected.includes("amplitude")) {
                    renderPlot(data, "amplitude");
                }
            });
        });
    }
});

window.addEventListener("DOMContentLoaded", () => {
    //let h5parm2 = get_h5parm_name();
    //console.log("Loading " + h5parm2);
    h5parm = "/home/ddkq81/test.h5";
    console.log("Loading " + h5parm);
    let solset_names = get_solset_names(h5parm).then(result => {
        const solset_list = document.getElementById('solset_picker');
        result.forEach(ss => {
            console.log("Adding solset " + ss);
            let solset = document.createElement("option");
            solset.value = ss;
            solset.textContent = ss;
            solset_list.appendChild(solset);
        });

        let ss_selected = solset_list.value;
        const soltab_list = document.getElementById('soltab_picker');
        let soltab_names = get_soltab_names(h5parm, ss_selected).then(result => {
            result.forEach(st => {
                console.log("Adding soltab " + st);
                let soltab = document.createElement("option");
                soltab.value = st;
                soltab.textContent = st;
                soltab_list.appendChild(soltab);
            });
            let st_selected = soltab_list.value;
            let station_names = get_station_names(h5parm, ss_selected, st_selected).then(result => {
                const list = document.getElementById('antennas');
                result.forEach(station => {
                    //console.log("Adding station " + station);
                    let button = document.createElement("div");
                    button.className = "option";
                    button.textContent = station;
                    button.addEventListener('click', () => {
                    let st = document.getElementById('antennas').getElementsByClassName("option");
                    for (var i = 0; i < st.length; i++) {
                          st.item(i).classList.remove("selected");
                      };
                      button.classList.toggle('selected');
                    });
                    list.appendChild(button);
                });
            });
        });
    });
});

