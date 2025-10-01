use crate::code_pair_put_back::CodePairPutBack;
use crate::enums::{BoundaryEdgeType, BoundaryPathType};
use crate::generated::entities::Hatch;
use crate::helper_functions::combine_points_2;
use crate::{CodePair, DxfError, DxfResult, Point, Vector};
use enum_primitive::FromPrimitive;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub enum BoundaryPath {
    Polyline(PolylineBoundaryData),
    Edge(EdgeBoundaryData),
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct HatchPatternBoundaryData {
    pub path_type: BoundaryPathType,
    pub path: BoundaryPath,
}

impl HatchPatternBoundaryData {
    pub(crate) fn read_boundary_paths_section(
        hatch: &mut Hatch,
        num_paths: i32,
        parser: &mut CodePairPutBack,
    ) -> DxfResult<bool> {
        for _ in 0..num_paths {
            let pair_92 = next_pair!(parser);
            let path_type: BoundaryPathType =
                enum_from_number!(BoundaryPathType, Default, from_i16, pair_92.assert_i16()?);
            if path_type == BoundaryPathType::Polyline {
                let _ = Self::read_polyline_boundary(hatch, path_type, parser)?;
            } else {
                let _ = Self::read_edge_boundary(hatch, path_type, parser)?;
            }
            //Source boundary object reading (TODO)
            let pair_97 = next_pair!(parser);
            if pair_97.code == 97 {
                let assoc_count: i32 = pair_97.assert_i32()?;
                for _ in 0..assoc_count {
                    let pair_330 = next_pair!(parser);
                    if pair_330.code != 330 {
                        let code = pair_330.code;
                        parser.put_back(Ok(pair_330));
                        return Err(DxfError::UnexpectedCode(code, 0));
                    }
                    let _ = pair_330.assert_string()?;
                }
            } else {
                parser.put_back(Ok(pair_97));
            }
        }
        Ok(true)
    }
    pub(crate) fn read_polyline_boundary(
        hatch: &mut Hatch,
        path_type: BoundaryPathType,
        parser: &mut CodePairPutBack,
    ) -> DxfResult<bool> {
        let mut polyline_data = PolylineBoundaryData {
            is_closed: false,
            vertices: Vec::new(),
        };
        let polyline_vertices_count: i32;
        loop {
            let pair = next_pair!(parser);

            match pair.code {
                72 => {
                    // Has Bulge flag (Optional)
                    // let has_bulge = pair.assert_i16()? != 0;
                }
                73 => {
                    // Is Closed flag (Opcional)
                    polyline_data.is_closed = pair.assert_i16()? != 0;
                }
                93 => {
                    polyline_vertices_count = pair.assert_i32()?;
                    break;
                }
                _ => {
                    let code = pair.code;
                    parser.put_back(Ok(pair));
                    return Err(DxfError::UnexpectedCode(code, 0));
                }
            }
        }
        if polyline_vertices_count > 0 {
            for _ in 0..polyline_vertices_count {
                let mut vertex = HatchPolylineVertex {
                    x: next_pair!(parser).assert_f64()?,
                    y: next_pair!(parser).assert_f64()?,
                    ..Default::default()
                };

                //Determines whether there's a bulge value.
                let next_pair = next_pair!(parser);
                if next_pair.code == 42 {
                    vertex.bulge = next_pair.assert_f64()?;
                } else {
                    parser.put_back(Ok(next_pair));
                }
                polyline_data.vertices.push(vertex);
            }
            let patern_data: Self = Self {
                path_type,
                path: BoundaryPath::Polyline(polyline_data),
            };
            hatch.pattern_boundary_data.push(patern_data);
        }
        Ok(true)
    }
    pub(crate) fn read_edge_boundary(
        hatch: &mut Hatch,
        path_type: BoundaryPathType,
        parser: &mut CodePairPutBack,
    ) -> DxfResult<bool> {
        let edges_count: i32 = next_pair!(parser).assert_i32()?;
        let mut edge_paths: Vec<EdgePath> = Vec::new();

        for _ in 0..edges_count {
            let edge_type: BoundaryEdgeType = enum_from_number!(
                BoundaryEdgeType,
                Line,
                from_i16,
                next_pair!(parser).assert_i16()?
            );

            match edge_type {
                BoundaryEdgeType::Line => {
                    let p1_x: f64 = next_pair!(parser).assert_f64()?;
                    let p1_y: f64 = next_pair!(parser).assert_f64()?;
                    let p2_x: f64 = next_pair!(parser).assert_f64()?;
                    let p2_y: f64 = next_pair!(parser).assert_f64()?;
                    let line_edge_data: EdgeLineData = EdgeLineData {
                        p1: Point {
                            x: p1_x,
                            y: p1_y,
                            z: 0.0,
                        },
                        p2: Point {
                            x: p2_x,
                            y: p2_y,
                            z: 0.0,
                        },
                    };
                    edge_paths.push(EdgePath::Line(line_edge_data));
                }
                BoundaryEdgeType::CicularArc => {
                    let center_x: f64 = next_pair!(parser).assert_f64()?; // Code 10
                    let center_y: f64 = next_pair!(parser).assert_f64()?; // Code 20
                    let radius: f64 = next_pair!(parser).assert_f64()?; // Code 40
                    let start_angle: f64 = next_pair!(parser).assert_f64()?; // Code 50
                    let end_angle: f64 = next_pair!(parser).assert_f64()?; // Code 51

                    let mut is_counter_clockwise: bool = false;

                    let next_pair = next_pair!(parser);
                    if next_pair.code == 73 {
                        is_counter_clockwise = next_pair.assert_i16()? != 0;
                    } else {
                        parser.put_back(Ok(next_pair));
                    }

                    let arc_data = EdgeCircularArcData {
                        center: Point {
                            x: center_x,
                            y: center_y,
                            z: 0.0,
                        },
                        radius,
                        start_angle,
                        end_angle,
                        is_counter_clockwise,
                    };
                    edge_paths.push(EdgePath::CircularArc(arc_data));
                }
                BoundaryEdgeType::EllipticArc => {
                    let center_x: f64 = next_pair!(parser).assert_f64()?; // Code 10
                    let center_y: f64 = next_pair!(parser).assert_f64()?; // Code 20
                    let major_axis_x: f64 = next_pair!(parser).assert_f64()?; // Code 11
                    let major_axis_y: f64 = next_pair!(parser).assert_f64()?; // Code 21
                    let minor_axis_ratio: f64 = next_pair!(parser).assert_f64()?; // Code 40
                    let start_angle: f64 = next_pair!(parser).assert_f64()?; // Code 50
                    let end_angle: f64 = next_pair!(parser).assert_f64()?; // Code 51

                    let mut is_counter_clockwise: bool = false;

                    let next_pair = next_pair!(parser);
                    if next_pair.code == 73 {
                        is_counter_clockwise = next_pair.assert_i16()? != 0;
                    } else {
                        parser.put_back(Ok(next_pair));
                    }

                    let elliptic_data = EdgeEllipticArcData {
                        center: Point {
                            x: center_x,
                            y: center_y,
                            z: 0.0,
                        },
                        major_axis: Vector {
                            x: major_axis_x,
                            y: major_axis_y,
                            z: 0.0,
                        },
                        minor_axis_ratio,
                        start_angle,
                        end_angle,
                        is_counter_clockwise,
                    };
                    edge_paths.push(EdgePath::EllipticArc(elliptic_data));
                }
                BoundaryEdgeType::Spline => {
                    Self::read_spline_edge_boundary(&mut edge_paths, parser)?;
                }
            }
        }

        let data: Self = Self {
            path_type,
            path: BoundaryPath::Edge(EdgeBoundaryData { edges: edge_paths }),
        };
        hatch.pattern_boundary_data.push(data);
        Ok(true)
    }
    pub(crate) fn read_spline_edge_boundary(
        edge_paths: &mut Vec<EdgePath>,
        parser: &mut CodePairPutBack,
    ) -> DxfResult<bool> {
        //Base properties (94, 73, 74)
        let mut spline_data = EdgeSplineData {
            degree: next_pair!(parser).assert_i16()?, // Code 94
            is_rational: next_pair!(parser).assert_i16()? != 0, // Code 73
            is_periodic: next_pair!(parser).assert_i16()? != 0, // Code 74
            knots: Vec::new(),
            weights: Vec::new(),
            control_points: Vec::new(),
            fit_points: Vec::new(),
            start_tangent: Vector {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            end_tangent: Vector {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            __control_point_x: Vec::new(),
            __control_point_y: Vec::new(),
            __fit_point_x: Vec::new(),
            __fit_point_y: Vec::new(),
        };

        //Knot counts (95, 96)
        let num_knots: i32 = next_pair!(parser).assert_i32()?; // Code 95
        let num_control_points: i32 = next_pair!(parser).assert_i32()?; // Code 96

        //Knot values (40)
        for _ in 0..num_knots {
            spline_data.knots.push(next_pair!(parser).assert_f64()?); // Code 40
        }

        for _ in 0..num_control_points {
            spline_data
                .__control_point_x
                .push(next_pair!(parser).assert_f64()?); // Code 10: X
            spline_data
                .__control_point_y
                .push(next_pair!(parser).assert_f64()?); // Code 20: Y

            // Weight - Code 42 (optional)
            let pair_42 = next_pair!(parser);
            if pair_42.code == 42 {
                spline_data.weights.push(Some(pair_42.assert_f64()?));
            } else {
                spline_data.weights.push(None);
                parser.put_back(Ok(pair_42));
            }
        }

        let num_fit_points: i32 = next_pair!(parser).assert_i32()?; // Code 97

        for _ in 0..num_fit_points {
            spline_data
                .__fit_point_x
                .push(next_pair!(parser).assert_f64()?); // Code 11: X
            spline_data
                .__fit_point_y
                .push(next_pair!(parser).assert_f64()?); // Code 21: Y
        }

        let start_tan_x: f64 = next_pair!(parser).assert_f64()?; // Code 12
        let start_tan_y: f64 = next_pair!(parser).assert_f64()?; // Code 22
        spline_data.start_tangent = Vector {
            x: start_tan_x,
            y: start_tan_y,
            z: 0.0,
        };

        let end_tan_x: f64 = next_pair!(parser).assert_f64()?; // Code 13
        let end_tan_y: f64 = next_pair!(parser).assert_f64()?; // Code 23
        spline_data.end_tangent = Vector {
            x: end_tan_x,
            y: end_tan_y,
            z: 0.0,
        };

        combine_points_2(
            &mut spline_data.__control_point_x,
            &mut spline_data.__control_point_y,
            &mut spline_data.control_points,
            Point::new, // Point::new(x, y, z)
        );

        combine_points_2(
            &mut spline_data.__fit_point_x,
            &mut spline_data.__fit_point_y,
            &mut spline_data.fit_points,
            Point::new,
        );

        edge_paths.push(EdgePath::Spline(spline_data));

        Ok(true)
    }
}

impl Default for HatchPatternBoundaryData {
    fn default() -> Self {
        Self {
            path_type: BoundaryPathType::Default,
            path: BoundaryPath::Polyline(PolylineBoundaryData {
                is_closed: false,
                vertices: Vec::new(),
            }),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct PolylineBoundaryData {
    pub is_closed: bool,
    pub vertices: Vec<HatchPolylineVertex>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct HatchPolylineVertex {
    pub x: f64,
    pub y: f64,
    pub bulge: f64,
}

impl Default for HatchPolylineVertex {
    fn default() -> HatchPolylineVertex {
        HatchPolylineVertex {
            x: 0.0,
            y: 0.0,
            bulge: 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub enum EdgePath {
    Line(EdgeLineData),
    CircularArc(EdgeCircularArcData),
    EllipticArc(EdgeEllipticArcData),
    Spline(EdgeSplineData), //TODO
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct EdgeBoundaryData {
    pub edges: Vec<EdgePath>,
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

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct EdgeSplineData {
    pub degree: i16,
    pub is_rational: bool,
    pub is_periodic: bool,
    pub knots: Vec<f64>,
    pub weights: Vec<Option<f64>>,
    pub control_points: Vec<Point>,
    pub fit_points: Vec<Point>,
    pub start_tangent: Vector,
    pub end_tangent: Vector,
    __control_point_x: Vec<f64>,
    __control_point_y: Vec<f64>,
    __fit_point_x: Vec<f64>,
    __fit_point_y: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct HatchPatternLineData {
    /// Pattern line angle (in degrees)
    pub angle: f64,
    /// Pattern line base point, X component
    pub base_point_x: f64,
    /// Pattern line base point, Y component
    pub base_point_y: f64,
    /// Pattern line offset, X component
    pub offset_x: f64,
    /// Pattern line offset, Y component
    pub offset_y: f64,
    /// Dash length
    pub dash_lengths: Vec<f64>,
}

impl Default for HatchPatternLineData {
    fn default() -> Self {
        Self {
            angle: 0.0,
            base_point_x: 0.0,
            base_point_y: 0.0,
            offset_x: 0.0,
            offset_y: 0.0,
            dash_lengths: Vec::new(),
        }
    }
}

impl HatchPatternLineData {
    pub(crate) fn read_pattern_line(
        hatch: &mut Hatch,
        pattern_line_count: i32,
        parser: &mut CodePairPutBack,
    ) -> DxfResult<bool> {
        let mut pattern_lines: Vec<Self> = Vec::new();
        for _ in 0..pattern_line_count {
            let angle: f64 = next_pair!(parser).assert_f64()?; // Code 53
            let base_point_x: f64 = next_pair!(parser).assert_f64()?; // Code 43: X
            let base_point_y: f64 = next_pair!(parser).assert_f64()?; // Code 44: Y
            let offset_x: f64 = next_pair!(parser).assert_f64()?; // Code 45: X
            let offset_y: f64 = next_pair!(parser).assert_f64()?; // Code 46: Y
            let num_dash_items: i16 = next_pair!(parser).assert_i16()?; // Code 79
            let mut dash_lengths: Vec<f64> = Vec::with_capacity(num_dash_items as usize);
            for _ in 0..num_dash_items {
                dash_lengths.push(next_pair!(parser).assert_f64()?);
            }
            let pattern_line_data = Self {
                angle,
                base_point_x,
                base_point_y,
                offset_x,
                offset_y,
                dash_lengths,
            };
            pattern_lines.push(pattern_line_data);
        }
        hatch.pattern_line_data = pattern_lines;
        Ok(true)
    }
}
