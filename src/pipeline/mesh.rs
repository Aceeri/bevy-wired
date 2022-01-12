pub use bevy::render::mesh::Mesh;

pub const ATTRIBUTE_BARYCENTRIC: &'static str = "Barycentric_Position";

pub trait ComputeBarycentric {
    fn compute_barycentric(&mut self);
}

impl ComputeBarycentric for Mesh {
    fn compute_barycentric(&mut self) {
        self.duplicate_vertices();

        let position_count = self
            .attribute(Mesh::ATTRIBUTE_POSITION)
            .expect("Need Mesh::ATTRIBUTE_POSITION to compute barycentric")
            .len();

        let barycentrics = [[0.0, 0.0, 1.0], [0.0, 1.0, 0.0], [1.0, 0.0, 1.0]];
        let barycentrics2 = [[0.0, 1.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 1.0]];
        let mut barycentric = Vec::new();
        for i in 0..position_count {
            let even = (i / 3) % 2 == 0;
            if even {
                barycentric.push(barycentrics[i % 3]);
            } else {
                barycentric.push(barycentrics2[i % 3]);
            }
        }

        self.set_attribute(ATTRIBUTE_BARYCENTRIC, barycentric);
    }
}

/*
module.exports.addBarycentricCoordinates = function addBarycentricCoordinates (bufferGeometry, removeEdge = false) {
  const attrib = bufferGeometry.getIndex() || bufferGeometry.getAttribute('position');
  const count = attrib.count / 3;
  const barycentric = [];

  // for each triangle in the geometry, add the barycentric coordinates
  for (let i = 0; i < count; i++) {
    const even = i % 2 === 0;
    const Q = removeEdge ? 1 : 0;
    if (even) {
      barycentric.push(
        0, 0, 1,
        0, 1, 0,
        1, 0, Q
      );
    } else {
      barycentric.push(
        0, 1, 0,
        0, 0, 1,
        1, 0, Q
      );
    }
  }
 */
