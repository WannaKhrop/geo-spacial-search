# 🌍 GeoSpatial Search

**GeoSpatial Search** is a Rust project focused on developing a fast and efficient algorithm for finding all neighboring points on the globe. It adapts the principles of KD-Trees to spherical geometry, enabling high-performance geospatial queries using great-circle distances.

This project has been integrated into the **Graph Database Query system** currently being developed at the **University of Bayreuth**, where it is used for geospatial indexing and proximity search on large-scale spatial data.

## ✨ Features

- Efficient search for nearby points based on geodesic (haversine) distance (radius search, box search) 
- KD-Tree-inspired structure adapted for spherical coordinates  
- Handles latitude and longitude inputs directly  
- Suitable for large-scale datasets and real-time applications  
- Designed for speed and accuracy in global proximity search