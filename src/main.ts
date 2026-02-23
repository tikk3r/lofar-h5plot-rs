//const { invoke } = window.__TAURI__.core;
import { invoke } from "@tauri-apps/api/core";

import * as Plot from '@observablehq/plot';

const plotContainer = document.getElementById('plotting_area');

const renderPlot = function(data: any, mode: string, dimension: any) {
    plot_dimensions = dimension;
    if (dimension.length == 1 && dimension[0] == "time") {
      const plot = Plot.plot({
        marks: [Plot.line(data, { x: 'x', y: 'y', stroke: 'blue' }),
            Plot.dot(data, { x: 'x', y: 'y', stroke: 'blue' }),
            Plot.crosshair(data, {x: 'x', y: "y"}),
        ],
        width: plotContainer!.clientWidth*0.85,
        height: plotContainer!.clientHeight*0.95,
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
          fontSize: "24"
        },
      });

      plotContainer!.innerHTML = '';
      plotContainer!.appendChild(plot);
    } else if (dimension.length == 1 && dimension[0] == "frequency") {
      const plot = Plot.plot({
        marks: [Plot.line(data, { x: 'x', y: 'y', stroke: 'blue' }),
            Plot.dot(data, { x: 'x', y: 'y', stroke: 'blue' }),
            Plot.crosshair(data, {x: 'x', y: "y", textFill: "white"})],
        width: plotContainer!.clientWidth*0.85,
        height: plotContainer!.clientHeight*0.95,
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
          fontSize: "24"
        },
      });

      plotContainer!.innerHTML = '';
      plotContainer!.appendChild(plot);
    } else if (dimension.length = 2) {
      const plot = Plot.plot({
        marks: [Plot.raster(
            data[0],
            {
              width: dimension[1],
              height: dimension[0],
              interpolate: "nearest",
            }),
        ],
        color: { scheme: mode == "phase" ? "sinebow" : "viridis" },
        width: plotContainer!.clientWidth,
        height: plotContainer!.clientHeight,
        marginBottom: 60,
        marginLeft: 90,
        marginTop: 50,
        x: {
            label: "Time [s]",
            tickRotate: -25,
            tickFormat: (d: number) => (data[1][d] - data[1][0]).toFixed(0),
        },
        y: {
          label: "Frequency [MHz]",
          tickFormat: (d: number) => (data[2][d] / 1e6).toFixed(1),
        },
        style: {
          width: '100%',
          height: '100%',
          maxWidth: 'none',
          maxHeight: 'none',
          objectFit: 'fill',
          fontSize: "24"
        },
      });
        
      plotContainer!.innerHTML = '';
      plotContainer!.appendChild(plot);
    }
};

window.addEventListener('resize', () => {
    const soltab_list = document.getElementById('soltab_picker');
    let st_selected = (soltab_list as HTMLInputElement).value;
    if (st_selected.includes("phase")) {
        mode = "phase";
    }else if (st_selected.includes("amplitude")) {
        mode = "amplitude";
    }
    if (plot_dimensions.length == 1){
        renderPlot(data, mode, plot_dimensions);
    }
});

async function get_h5parm_name(): Promise<string> {
    return await invoke("get_h5parm_name");
}

async function get_station_names(h5parm: string, ss: string, st: string): Promise<string[]> {
    return await invoke("get_station_names", {h5: h5parm, solset: ss, soltab: st});
}

async function get_solset_names(h5parm: string): Promise<string[]> {
    return await invoke("get_solset_names", {h5: h5parm});
}

async function get_soltab_names(h5parm: string, ss: string): Promise<string[]> {
    return await invoke("get_soltab_names", {h5: h5parm, solset: ss});
}

async function get_soltab_times(h5parm: string, ss: string, st: string): Promise<number[]> {
    return await invoke("get_soltab_times", {h5: h5parm, solset: ss, soltab: st});
}

async function get_soltab_freqs(h5parm: string, ss: string, st: string): Promise<number[]> {
    return await invoke("get_soltab_freqs", {h5: h5parm, solset: ss, soltab: st});
}

async function get_values_time(h5parm: string, ss: string, st: string, antenna: string, refant: string, chan: number): Promise<number[]> {
    return await invoke("get_values_time", {h5: h5parm, solset: ss, soltab: st, antenna: antenna, refant: refant, channel: chan});
}

async function get_values_frequency(h5parm: string, ss: string, st: string, antenna: string, refant: string): Promise<number[]> {
    return await invoke("get_values_frequency", {h5: h5parm, solset: ss, soltab: st, antenna: antenna, refant: refant, freqdiff: freqdiff});
}

async function get_values_waterfall(h5parm: string, ss: string, st: string, antenna: string, refant: string): Promise<number[]> {
    return await invoke("get_values_waterfall", {h5: h5parm, solset: ss, soltab: st, antenna: antenna, refant: refant});
}

function update_channel_picker(max: number) {
    const slider = document.getElementById("channel_picker");
    slider.setAttribute("min", 0);
    slider.setAttribute("max", max);
}

let h5parm: string = "";
let mode: string = "phase";
let ax_selected: string = "time";
let data: any = null;
let plot_dimensions: string = "time";
let channel: number = 0;
let freqdiff: boolean = false;

document.getElementById("button_plot")!.addEventListener('click', () => {
    const solset_list = document.getElementById('solset_picker');
    let ss_selected: string = (solset_list as HTMLInputElement).value;
    const soltab_list = document.getElementById('soltab_picker');
    let st_selected: string = (soltab_list as HTMLInputElement).value;
    const antenna = document.getElementById('antennas')!.getElementsByClassName("selected")[0].innerHTML;

    const axis_list = document.getElementById('axis_picker');
    ax_selected = (axis_list as HTMLInputElement).value;
    if (ax_selected == "time") {
        get_soltab_times(h5parm, ss_selected, st_selected).then((times: number[]) => {
            channel = parseFloat(document.getElementById("channel_picker").value);
            get_values_time(h5parm, ss_selected, st_selected, antenna, "CS002HBA0", channel).then((values: number[]) => {
                data = times.map((value: number, index: number) => ({
                  x: value - times[0],
                  y: values[index],
                }));
                if (st_selected.includes("phase")) {
                    renderPlot(data, "phase", ["time"]);
                }else if (st_selected.includes("amplitude")) {
                    renderPlot(data, "amplitude", ["time"]);
                }
            });
        });
    } else if (ax_selected == "frequency") {
        get_soltab_freqs(h5parm, ss_selected, st_selected).then(freqs => {
            get_values_frequency(h5parm, ss_selected, st_selected, antenna, "CS002HBA0").then(values => {
                data = freqs.map((value, index) => ({
                  x: value / 1e6,
                  y: values[index],
                }));
                if (st_selected.includes("phase")) {
                    renderPlot(data, "phase", ["frequency"]);
                }else if (st_selected.includes("amplitude")) {
                    renderPlot(data, "amplitude", ["frequency"]);
                }
            });
        });
    } else if (ax_selected == "waterfall") {
        get_soltab_times(h5parm, ss_selected, st_selected).then((times: number[]) => {
            get_soltab_freqs(h5parm, ss_selected, st_selected).then((freqs: number[]) => {
                get_values_waterfall(h5parm, ss_selected, st_selected, antenna, "CS002HBA0").then(result => {
                    let data = [result[0], times, freqs];
                    let width: number = result[1];
                    let height: number = result[2];
                    if (st_selected.includes("phase")) {
                        renderPlot(data, "phase", [width, height]);
                    }else if (st_selected.includes("amplitude")) {
                        renderPlot(data, "amplitude", [width, height]);
                    }
                });
            });
        });
    }
});

document.getElementById("plot_freqdiff").addEventListener('change', () => {
    document.getElementById("plot_freqdiff").selected ^= 1;
    freqdiff = Boolean(document.getElementById("plot_freqdiff").selected);
    console.log(document.getElementById("plot_freqdiff").selected);
});

window.addEventListener("DOMContentLoaded", () => {
    get_h5parm_name().then((h5: string) => {
        h5parm = h5;
        get_solset_names(h5).then(result => {
            const solset_list = document.getElementById('solset_picker');
            result.forEach(ss => {
                let solset = document.createElement("option");
                solset.value = ss;
                solset.textContent = ss;
                solset_list!.appendChild(solset);
            });

            let ss_selected = (solset_list as HTMLInputElement).value;
            const soltab_list = document.getElementById('soltab_picker');
            get_soltab_names(h5, ss_selected).then(result => {
                result.forEach(st => {
                    let soltab = document.createElement("option");
                    soltab.value = st;
                    soltab.textContent = st;
                    soltab_list!.appendChild(soltab);
                });
                let st_selected = (soltab_list as HTMLInputElement).value;
                get_station_names(h5, ss_selected, st_selected).then(result => {
                    let list = document.getElementById('antennas');
                    result.forEach(station => {
                        let button = document.createElement("div");
                        button.className = "option";
                        button.textContent = station;
                        button.addEventListener('click', () => {
                        let st = document.getElementById('antennas')!.getElementsByClassName("option");
                        for (var i = 0; i < st.length; i++) {
                              st.item(i)!.classList.remove("selected");
                          };
                          button.classList.toggle('selected');
                        });
                        list!.appendChild(button);
                    });
                });
            });
        });
    });
});

