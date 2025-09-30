use crate::enums::{BoundaryEdgeType, BoundaryPathType, HatchStyle, PatternType};
use crate::generated::entities::Vertex;
use crate::point::Point;
use crate::vector::Vector;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub enum BoundaryPath {
    Polyline(PolylineBoundaryData),
    Edge(EdgeBoundaryData),
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct HatchPatternData {
    pub paths: Vec<BoundaryPath>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct PolylineBoundaryData {
    pub is_closed: bool,
    pub vetices: Vec<Vertex>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub enum EgdePath {
    Line(EdgeLineData),
    CircularArc(EdgeCircularArcData),
    EllipticArc(EdgeEllipticArcData),
    //Spline(EdgeSplineData), //TODO
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct EdgeBoundaryData {
    pub edge: EgdePath,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct EdgeLineData {
    pub p1: Point,
    pub p2: Point,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct EdgeCircularArcData {
    pub center: Point,
    pub radius: f64,
    pub start_angle: f64,
    pub end_angle: f64,
    pub is_counter_clockwise: bool,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct EdgeEllipticArcData {
    pub center: Point,
    pub major_axis: Vector,
    pub minor_axis_ratio: f64,
    pub start_angle: f64,
    pub end_angle: f64,
    pub is_counter_clockwise: bool,
}

//ToDo
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct EdgeSplineData {}
