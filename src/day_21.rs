use axum::extract::Path;
use dms_coordinates::{DMS3d, DMS};
use s2::{cell::Cell, cellid::CellID};

pub async fn task_1(Path(binary): Path<String>) -> String {
    let s2_cell_id = CellID(u64::from_str_radix(&binary, 2).unwrap());
    let center = Cell::from(s2_cell_id).center();
    let dms = DMS3d::from_decimal_degrees(center.latitude().deg(), center.longitude().deg(), None);

    dms.format()
}

trait Format {
    fn format(&self) -> String;
}

impl Format for DMS3d {
    fn format(&self) -> String {
        format!("{} {}", self.latitude.format(), self.longitude.format())
    }
}

impl Format for DMS {
    fn format(&self) -> String {
        format!(
            "{}°{}'{:.3}''{}",
            self.degrees, self.minutes, self.seconds, self.bearing
        )
    }
}
