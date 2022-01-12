pub mod pipeline;

pub use pipeline::*;

pub mod prelude {
    pub use crate::pipeline::{
        StylizedWireframe, StylizedWireframePipeline, StylizedWireframePlugin,
        SimpleWireframe, SimpleWireframePipeline, SimpleWireframePlugin,
        ATTRIBUTE_BARYCENTRIC, ComputeBarycentric,
    };
}
