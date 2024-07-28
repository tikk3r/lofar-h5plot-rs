use slint;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let stations: Vec<slint::StandardListViewItem> = vec![
        "CS001HBA0".into(),
        "CS001HBA1".into(),
        "CS002HBA0".into(),
        "CS002HBA1".into(),
        "CS003HBA0".into(),
        "CS003HBA1".into(),
        "CS004HBA0".into(),
        "CS004HBA1".into(),
        "CS005HBA0".into(),
        "CS005HBA1".into(),
        "CS006HBA0".into(),
        "CS006HBA1".into(),
        "CS007HBA0".into(),
        "CS007HBA1".into(),
    ];
    let stations_model = std::rc::Rc::new(slint::VecModel::from(stations));

    let refants: Vec<slint::SharedString> = vec![
        "CS001HBA0".into(),
        "CS001HBA1".into(),
        "CS002HBA0".into(),
        "CS002HBA1".into(),
        "CS003HBA0".into(),
        "CS003HBA1".into(),
        "CS004HBA0".into(),
        "CS004HBA1".into(),
        "CS005HBA0".into(),
        "CS005HBA1".into(),
        "CS006HBA0".into(),
        "CS006HBA1".into(),
        "CS007HBA0".into(),
        "CS007HBA1".into(),
    ];
    let refant_model = std::rc::Rc::new(slint::VecModel::from(refants));

    ui.set_station_list(stations_model.into());
    ui.set_refant_list(refant_model.into());

    ui.on_request_increase_value({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_counter(ui.get_counter() + 1);
        }
    });

    ui.run()
}
