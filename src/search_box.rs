use crate::geo_point::Point;

/*
this structure can describe each tree node. It describes a subspace of a tree node. 
Will be used for search to calculate disances
*/
#[derive(Debug, Clone, Copy)]
pub struct SearchBox {
    pub lat_from: f64,
    pub lat_to: f64,
    pub lon_from: f64,
    pub lon_to: f64
}

impl SearchBox {

    pub fn new(lat_from: f64, lat_to: f64, lon_from: f64, lon_to: f64) -> Self {
        return SearchBox {
            lat_from,
            lat_to,
            lon_from,
            lon_to
        };
    }

    pub fn is_inside(&self, point: &Point) -> bool {
        let cond_1 = self.lat_from <= point.lat;
        let cond_2 = point.lat <= self.lat_to;
        let cond_3 = self.lon_from <= point.lon;
        let cond_4 = point.lon <= self.lon_to;

        return cond_1 && cond_2 && cond_3 && cond_4;
    }

    pub fn nested_box(box_internal: &Self, box_external: &Self) -> bool {

        let cond_1 = box_external.lat_from <= box_internal.lat_from;
        let cond_2 = box_internal.lat_to <= box_external.lat_to;
        let cond_3 = box_external.lon_from <= box_internal.lon_from;
        let cond_4 = box_internal.lon_to <= box_external.lon_to;

        return cond_1 && cond_2 && cond_3 && cond_4;
    }
}