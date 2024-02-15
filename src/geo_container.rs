use num_traits::ToPrimitive;
use serde::{Deserialize, Serialize};

use super::{geo_search_trait::GeoSearch, sphere_helper::SphereHelper, geo_point::Point};

#[derive(Debug, Serialize, Deserialize)]
/// Data structure for solving geosearch problem in case of small amount of points
/// In this case simple iterative search is applied
pub struct GeoContainer {
    /// Id of the globe the data structure was constructed for
    globe: u64,
    /// Radius of the globe. Measure unit = km.
    sphere_radius: f64,
    /// Geopoints that belong to the globe
    points: Vec::<Point>,
}

impl GeoContainer {
    /// Create a new instance of GeoContainer
    /// 
    /// ## Arguments
    /// 
    /// * 'points' - Geopoints that belong to the globe
    /// * 'sphere_radius' - Radius of the globe. Measure unit = km.
    /// * 'globe' - Id of the globe the data structure was constructed for
    /// 
    /// ## Returns
    /// 
    /// * 'container' - New instance of GeoContainer 
    pub fn new(points: &Vec::<Point>, sphere_radius: f64, globe: u64) -> Self {

        let data_points = points.iter().map(|point| point.clone()).collect();

        return GeoContainer {
            globe: globe,
            sphere_radius: sphere_radius,
            points: data_points,
        };
    }
}

impl GeoSearch for GeoContainer {
    fn search_by_distance<R: Relation + FromIterator<Vec<Node>>>(&self, point: &Point, distance: f64) -> R {
        // Filter candidates and get the answer
        let results = self.points.iter().filter(|candidate| {
            SphereHelper::distance(point, candidate, self.sphere_radius) <= distance
        }).map(|candidate| {

            // Return point and distance to this point in km.
            let distance_to_candidate = SphereHelper::distance(point, &candidate, self.sphere_radius).to_f32().unwrap();

            // Return a relation with 2 columns => (Node.id, Distance)
            let mut vec = Vec::with_capacity(2);
            vec.push(candidate.node);
            vec.push(from_number_float(distance_to_candidate, NodeType::NumberFloat));
            return vec;
        }).collect();

        return results;
    }

    fn search_by_box<R: Relation + FromIterator<Vec<Node>>>(&self, corner_west: &Point, corner_east: &Point) -> R {
        
        // Identify SearchBoxes for the target
        let (box_0, box_1) = SphereHelper::construct_searchbox(corner_west, corner_east);

        // Filter candidates and get the answer
        let results = self.points.iter().filter(|candidate| {
            match (box_0, box_1) {
                (Some(target_0), Some(target_1)) => {
                    target_0.is_inside(&candidate) || target_1.is_inside(&candidate)
                },
                (Some(target_0), None) => {
                    target_0.is_inside(&candidate)
                },
                _ => {false}
            }
        }).map(|point| {
            let mut vec = Vec::with_capacity(1);
            vec.push(point.node);
            return vec;
        }).collect();

        return results;
    }
}