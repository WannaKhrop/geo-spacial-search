use std::f64::consts::PI;
use optimization::{Minimizer, GradientDescent, NumericalDifferentiation, Func};

use crate::geo_point::Point;
use crate::search_box::SearchBox;

pub struct SphereHelper {
    // this struct does not have any field
    // static methods only
}

impl SphereHelper {

    // calculate haversine of value https://en.wikipedia.org/wiki/Versine#ahav
    pub fn hav(x: f64) -> f64 {
        return (1.0 - f64::cos(x)) / 2.0;
    }

    // calculate archaversine of value https://en.wikipedia.org/wiki/Versine#ahav
    pub fn archav(h: f64) -> f64 {
        return if h >= 0.0 && h <= 1.0 {f64::acos(1.0 - 2.0 * h)} else {0.0};
    }

    // calculate spherical distance https://en.wikipedia.org/wiki/Haversine_formula
    pub fn distance(p1: &Point, p2: &Point, radius: f64) -> f64 {

        let d_lat: f64 = p1.lat - p2.lat;
        let d_lon: f64 = p1.lon - p2.lon;

        // just apply formula 
        let angle = Self::archav(Self::hav(d_lat) + f64::cos(p1.lat) * f64::cos(p2.lat) * Self::hav(d_lon));

        return angle * radius;
    }
 
    pub fn find_box(point: &Point, distance: f64, radius: f64) -> (Option::<SearchBox>, Option::<SearchBox>) {

        let d_lat = distance / radius;

        // in case near the nord pole => look around the pole
        if point.lat + d_lat >= PI / 2.0 {
            return (
                Some(SearchBox { 
                    lat_from: f64::max(point.lat - d_lat, -PI / 2.0),
                    lat_to: PI / 2.0,
                    lon_from: -PI,
                    lon_to: PI
                }),
                None
            );
        }

        // in case near the south pole => look around the pole
        if point.lat - d_lat <= -PI / 2.0 {
            return (
                Some(SearchBox {
                    lat_from: -PI / 2.0,
                    lat_to: f64::min(point.lat + d_lat, PI / 2.0),
                    lon_from: -PI,
                    lon_to: PI
                }),
                None
            );
        }

        // minimization block ---------------------------------------------------------------------------------
        let d_lon_function = NumericalDifferentiation::new(Func(|x: &[f64]| {
            let dividend = SphereHelper::hav(d_lat) - SphereHelper::hav(x[0] - point.lat);
            let divisor = f64::cos(x[0]) * f64::cos(point.lat);

            return if f64::abs(divisor) >= f64::abs(dividend) {-1.0 * SphereHelper::archav(dividend / divisor)} else {0.0};
        }));
        let minimizer = GradientDescent::new().max_iterations(Some(1000));
        let solution = minimizer.minimize(&d_lon_function, vec![point.lat]);

        // ----------------------------------------------------------------------------------------------------

        // identify maximal possible diffrence in longitude
        let d_lon = f64::abs(solution.value);

        // if the distance is so large that we can go over the sphere => look over sphere
        if d_lon >= PI {
            return (
                Some(SearchBox {
                    lat_from: point.lat - d_lat,
                    lat_to: point.lat + d_lat,
                    lon_from: -PI,
                    lon_to: PI
                }),
                None
            );
        }

        // if the point if near the right border => look near the left border, because it's a sphere
        if point.lon + d_lon > PI {

            let delta = point.lon + d_lon - PI;

            return (
                Some(SearchBox {
                    lat_from: point.lat - d_lat,
                    lat_to: point.lat + d_lat,
                    lon_from: point.lon - d_lon,
                    lon_to: PI
                }),
                Some(SearchBox {
                    lat_from: point.lat - d_lat,
                    lat_to: point.lat + d_lat,
                    lon_from: -PI,
                    lon_to: f64::min(delta - PI, point.lon - d_lon)
                })
            );
        }

        // if the point if near the left border => look near the right border, because it's a sphere
        if point.lon - d_lon < -PI {

            let delta = d_lon - point.lon - PI;

            return (
                Some(SearchBox {
                    lat_from: point.lat - d_lat,
                    lat_to: point.lat + d_lat,
                    lon_from: -PI,
                    lon_to: point.lon + d_lon
                }),
                Some(SearchBox {
                    lat_from: point.lat - d_lat,
                    lat_to: point.lat + d_lat,
                    lon_from: f64::max(PI - delta, point.lon + d_lon),
                    lon_to: PI
                })
            );
        }

        // in other cases just return one SearchBox
        return (
            Some(SearchBox { 
                lat_from: point.lat - d_lat,
                lat_to: point.lat + d_lat,
                lon_from: point.lon - d_lon,
                lon_to: point.lon + d_lon
            }),
            None
        );
    }

}