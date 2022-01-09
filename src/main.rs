mod model;
mod ui;

use crate::model::rpn_calc::RpnCalc;
use simple_logger::SimpleLogger;
use std::error::Error;

use crate::ui::hello_gui::HelloGui;

fn main() -> Result<(), Box<dyn Error>> {
    SimpleLogger::new()
        .with_utc_timestamps()
        .with_colors(true)
        .init()?;
    log::info!("Hello World!");

    let mut app = HelloGui::new(RpnCalc::new());
    app.run();

    log::info!(
        "Exiting normally, model value is {}",
        app.get_model().to_string().replace("\n", ", ")
    );
    Ok(())
}
