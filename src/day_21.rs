use axum::extract::{Path, State};
use country_boundaries::{CountryBoundaries, LatLon};
use dms_coordinates::{DMS3d, DMS};
use iso_country::Country;
use s2::{cell::Cell, cellid::CellID};
use std::sync::Arc;

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

pub async fn task_2(
    Path(binary): Path<String>,
    State(geocoder): State<Arc<CountryBoundaries>>,
) -> String {
    let s2_cell_id = CellID(u64::from_str_radix(&binary, 2).unwrap());
    let center = Cell::from(s2_cell_id).center();

    let search_result =
        geocoder.ids(LatLon::new(center.latitude().deg(), center.longitude().deg()).unwrap());

    let alpha2_country_code = search_result.iter().last().unwrap();
    let country: Country = alpha2_country_code.parse().unwrap();

    match country {
        Country::BN => "Brunei",
        country => country.name(),
    }
    .to_string()
}
