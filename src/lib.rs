pub mod pipeline;

pub use pipeline::*;

pub mod prelude {
    pub use crate::pipeline::{
        StylizedWireframe, StylizedWireframePipeline, StylizedWireframePlugin,
        ATTRIBUTE_BARYCENTRIC, ComputeBarycentric,
    };
}
