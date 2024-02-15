use crate::{geo_point::Point, sphere_helper::SphereHelper, search_box::SearchBox};
use std::f64::consts::PI;
use std::cmp::Ordering;

/*
KD-Tree node = one node of a KD-Tree
It can be a intermediate node without large amount of data
Or it can be a leaf with data points
*/
#[derive(Debug, Clone)]
enum KDTreeNode {

    Node {
        splitter: f64, // the value which was used to split points in two parts
        dimension: usize, // which dimension was used for split. 0 => latitude, 1 => lontitude
        left_child: usize, // index of the left child in array
        right_child: usize, // index of the right child in array
    },
    Leaf {
        points: Vec::<Point>
    }
}


//  tree structure
#[derive(Debug)]
pub struct KDTree {
    root: usize, // index of root in the array of nodes
    nodes: Vec<KDTreeNode>, // array of all nodes to get rid of pointers
    sphere_radius: f64 // radius of the sphere that describes points
}

/*
kd-tree idead = https://en.wikipedia.org/wiki/K-d_tree
here can be found algorithms that basically ensure fast search in kd-tree
*/
impl KDTree {

    // n_stop parameter to get rid of splitting dense areas => we don't want to create a tree very deep 
    pub fn new(points: &Vec::<Point>, n_stop: usize, sphere_radius: f64) -> Self {

        // default initialization
        // nodes = array of all tree nodes that will be constructed 
        let mut nodes = Vec::new();
        let mut indices: Vec<usize> = (0..points.len()).collect();

        // build the tree
        let idx = Self::build(&mut nodes, &mut indices, &points, n_stop);
        return KDTree { 
            root: idx,
            nodes: nodes,
            sphere_radius: sphere_radius
        };
    }

    fn build(nodes: &mut Vec<KDTreeNode>, indices: &mut Vec<usize>, points: &Vec::<Point>, n_stop: usize) -> usize {

        // if number of points is small enough => then it's a leaf else build node !!!
        if indices.len() <= n_stop {
            let idx = nodes.len() as usize;
            nodes.push(KDTreeNode::Leaf { 
                points: indices.iter().map(|&i| points[i].clone()).collect()
            });
            return idx;
        } 
        
        // identify dimension for split
        let dimension = Self::choose_dimension(points, indices);
        indices.sort_by(|&a, &b| {
            if dimension == 0 {
                points[a].lat.partial_cmp(&points[b].lat).unwrap_or(Ordering::Equal)
            } else {
                points[a].lon.partial_cmp(&points[b].lon).unwrap_or(Ordering::Equal)
            }
        });
        
        // get median point for better split. We do not want point on border.
        let median_index = (indices.len() - 1) / 2;

        // identify value that splits points in two arrays
        let split_value = if dimension == 0 { 
            (points[indices[median_index]].lat + points[indices[median_index + 1]].lat) / 2.0
        } 
        else {
            (points[indices[median_index]].lon + points[indices[median_index + 1]].lon) / 2.0
        };
        
        // build childs and node itself
        let node = KDTreeNode::Node {
            splitter: split_value,
            dimension: dimension,
            left_child: Self::build(nodes, &mut indices[..median_index + 1].to_vec(), &points, n_stop),
            right_child: Self::build(nodes, &mut indices[median_index + 1..].to_vec(), &points, n_stop),
        };
        
        // add node to the vector
        let idx = nodes.len() as usize;
        nodes.push(node);

        return idx;
    }

    fn choose_dimension(points: &Vec::<Point>, indices: &Vec<usize>) -> usize {
        let x_values: Vec<f64> = indices.iter().map(|&i| points[i].lat).collect();
        let y_values: Vec<f64> = indices.iter().map(|&i| points[i].lon).collect();

        if Self::calculate_variance(&x_values) > Self::calculate_variance(&y_values) {
            return 0; // Choose x dimension
        } else {
            return 1; // Choose y dimension
        }
    }

    fn calculate_variance(values: &Vec<f64>) -> f64 {
        let mean = values.iter().sum::<f64>() / values.len() as f64;
        values.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / values.len() as f64
    }

    pub fn insert(&mut self, point: &Point) {
        // Implementation of insertion goes here
        // You would need to modify the vector and potentially update the tree structure
        // based on the point's position
    }

    pub fn search_by_distance(&self, point: &Point, distance: f64) -> Vec::<Point> {

        let (box_0, box_1) = SphereHelper::find_box(&point, distance, self.sphere_radius);
        let initial_box = SearchBox {
            lat_from: -PI / 2.0,
            lat_to: PI / 2.0,
            lon_from: -PI,
            lon_to: PI
        };

        // collect candidates for results
        let mut candidates = Vec::<Point>::new();

        if let Some(target) = box_0 {
            candidates.extend(self.search(self.root, &initial_box, &target))
        }

        if let Some(target) = box_1 {
            candidates.extend(self.search(self.root, &initial_box, &target));
        }

        // filter candidates and get the answers
        let results = candidates.into_iter().filter(|candidate| {
            SphereHelper::distance(point, candidate, self.sphere_radius) <= distance
        }).collect();

        return results;
    }

    fn search(&self, node_idx: usize, current_box: &SearchBox, target: &SearchBox) -> Vec::<Point> {

        if SearchBox::nested_box(&current_box, &target) {
            return self.extract_all(node_idx);
        }

        match &self.nodes[node_idx] {
            KDTreeNode::Leaf { points } => {
                return points.into_iter().filter(|&point| target.is_inside(&point)).map(|point| point.clone()).collect();
            },
            &KDTreeNode::Node { splitter, dimension, left_child, right_child } => {

                // search using latitude as split criteria in the node
                let mut result = Vec::<Point>::new();

                if dimension == 0 {
                    if target.lat_from <= splitter && splitter < target.lat_to {
                        // here we have to split our search in two parts and search in both directions
                        
                        // left child
                        result.extend(self.search(left_child, 
                            &SearchBox {
                                lat_from: current_box.lat_from,
                                lat_to: splitter,
                                lon_from: current_box.lon_from,
                                lon_to: current_box.lon_to
                            }, 
                            &SearchBox {
                                lat_from: target.lat_from,
                                lat_to: splitter,
                                lon_from: target.lon_from,
                                lon_to: target.lon_to
                            }
                        ));

                        // right child
                        result.extend(self.search(right_child, 
                            &SearchBox {
                                lat_from: splitter,
                                lat_to: current_box.lat_to,
                                lon_from: current_box.lon_from,
                                lon_to: current_box.lon_to
                            },
                            &SearchBox {
                                lat_from: splitter,
                                lat_to: target.lat_to,
                                lon_from: target.lon_from,
                                lon_to: target.lon_to
                            }
                        ));

                    }
                    else if target.lat_from < splitter && target.lat_to <= splitter {
                        // search just in the left subtree
                        result.extend(self.search(left_child, 
                            &SearchBox {
                                lat_from: current_box.lat_from,
                                lat_to: splitter,
                                lon_from: current_box.lon_from,
                                lon_to: current_box.lon_to
                            }, 
                            target
                        ));
                    }
                    else {
                        // search just in the right subtree
                        result.extend(self.search(right_child, 
                            &SearchBox {
                                lat_from: splitter,
                                lat_to: current_box.lat_to,
                                lon_from: current_box.lon_from,
                                lon_to: current_box.lon_to
                            },
                            target
                        ));
                    }
                }
                // search using lontitude as split criteria in the node
                else {
                    
                    if target.lon_from <= splitter && splitter < target.lon_to {
                        // here we have to split our search in two parts and search in both directions
                        
                        // left child
                        result.extend(self.search(left_child, 
                            &SearchBox {
                                lat_from: current_box.lat_from,
                                lat_to: current_box.lat_to,
                                lon_from: current_box.lon_from,
                                lon_to: splitter
                            }, 
                            &SearchBox {
                                lat_from: target.lat_from,
                                lat_to: target.lat_to,
                                lon_from: target.lon_from,
                                lon_to: splitter
                            }
                        ));

                        // right child
                        result.extend(self.search(right_child, 
                            &SearchBox {
                                lat_from: current_box.lat_from,
                                lat_to: current_box.lat_to,
                                lon_from: splitter,
                                lon_to: current_box.lon_to
                            },
                            &SearchBox {
                                lat_from: target.lat_from,
                                lat_to: target.lat_to,
                                lon_from: splitter,
                                lon_to: target.lon_to
                            }
                        ));
                    }
                    else if target.lon_from < splitter && target.lon_to <= splitter {
                        // search just in the left subtree
                        result.extend(self.search(left_child, 
                            &SearchBox {
                                lat_from: current_box.lat_from,
                                lat_to: current_box.lat_to,
                                lon_from: current_box.lon_from,
                                lon_to: splitter
                            }, 
                            target
                        ));
                    }
                    else {
                        // search just in the right subtree
                        result.extend(self.search(right_child, 
                            &SearchBox {
                                lat_from: current_box.lat_from,
                                lat_to: current_box.lat_to,
                                lon_from: splitter,
                                lon_to: current_box.lon_to
                            },
                            target
                        ));

                    }
                }

                return result;
            }
        }
    }

    fn extract_all(&self, node_idx: usize) -> Vec<Point> {

        match &self.nodes[node_idx] {
            &KDTreeNode::Node { splitter: _, dimension: _, left_child, right_child } => {

                let mut result = Vec::<Point>::new();

                // collect everything from left child
                result.extend(self.extract_all(left_child));

                // collect everything from right child
                result.extend(self.extract_all(right_child));

                return result;
            },
            KDTreeNode::Leaf { points } => {
                return points.iter().map(|&point| point.clone()).collect();
            }
        }
    }

}