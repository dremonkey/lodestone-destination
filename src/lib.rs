/// The main crate for lodestone-destination
/// 
/// ## Overview
/// 
/// Calculates the destination point given a starting point, distance, and initial
/// bearing.
/// 
/// # Arguments
/// * `point` - FeaturePoint
/// * `distance` - distance in (degrees | kilometers | meters | miles | radians)
/// * `bearing` - initial bearing in degrees
/// * `units` - unit of measurement for distance


// Third party crates
extern crate lodestone_core;
extern crate lodestone_point;

use lodestone_point::FeaturePoint;
use lodestone_core::{utils, wgs84};

pub extern fn destination(
    point: &FeaturePoint, 
    distance: f64,
    bearing: f64,
    units: &str) -> FeaturePoint {

  let coord = point.coordinates();
  let lat = coord[1].to_radians();
  let lng = coord[0].to_radians();
  let bearing_rad = bearing.to_radians();

  let radius = match units {
    "degrees" => 1.0_f64.to_degrees(),
    "kilometers" | "km" => wgs84::RADIUS / 1000.0,
    "meters" | "m" => wgs84::RADIUS,
    "miles" | "mi" => utils::km_to_mi(wgs84::RADIUS / 1000.0),
    "radians" => 1.0,
    _ => panic!("Unknown unit of measurement: {}", units)
  };

  let dlat = (lat.sin() * (distance / radius).cos() +
              lat.cos() * (distance / radius).sin() * bearing_rad.cos()).asin();
  let dlng = lng + 
             (bearing_rad.sin() * (distance / radius).sin() * lat.cos()).atan2(
              (distance / radius).cos() - lat.sin() * dlat.sin()
             );

  FeaturePoint::new(vec![dlng.to_degrees(), dlat.to_degrees()])
}

#[cfg(test)]
mod tests {
  use lodestone_point::FeaturePoint;
  use super::destination;

  #[test]
  #[should_panic(expected = "Unknown unit of measurement")]
  fn test_wrong_units() {
    let sf = vec![-122.4167,37.7833];
    let sf_point = FeaturePoint::new(sf);
    destination(&sf_point, 100.0, 50.0, "leagues");
  }

  #[test]
  fn test_simple() {
    let pt1 = FeaturePoint::new(vec![0.0, 0.0]);
    let dist = 55.6; // kilometers
    let bearing = 90.0;

    // expected
    let pt2 = FeaturePoint::new(vec![0.4994633, 0.0]);

    let dest = destination(&pt1, dist, bearing, "km");
    assert_eq!(dest, pt2);
  }
  
  #[test]
  fn test_from_sf_using_kilometers() {
    let sf = vec![-122.4167,37.7833];
    let sf_point = FeaturePoint::new(sf);
    let distance = 4133.177968880825; // distance to ny in km
    let bearing = 69.91944547551958;

    // expected
    let ny = vec![-74.0059,40.7127];
    let ny_point = FeaturePoint::new(ny);
    
    // calculate
    let dest = destination(&sf_point, distance, bearing, "km");

    assert_eq!(dest, ny_point);
  }

  #[test]
  fn test_from_sf_using_miles() {
    let sf = vec![-122.4167,37.7833];
    let sf_point = FeaturePoint::new(sf);
    let distance = 2568.236927701447; // distance to ny in miles
    let bearing = 69.91944547551958;

    // expected
    let ny = vec![-74.0059,40.7127];
    let ny_point = FeaturePoint::new(ny);
    
    // calculate
    let dest = destination(&sf_point, distance, bearing, "mi");

    assert_eq!(dest, ny_point);
  }
}
