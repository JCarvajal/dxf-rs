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
        loop_count: &mut i32,
        iter: &mut CodePairPutBack,
    ) -> DxfResult<()> {
        while *loop_count > 0 {
            let mut path_type: Option<BoundaryPathType> = None;
            let mut boundary_data_read = false;
            let mut source_boundary_objects_count: i32 = 0;
            loop {
                let pair = match iter.next() {
                    Some(Ok(pair)) => pair,
                    Some(Err(e)) => return Err(e),
                    None => return Err(DxfError::UnexpectedEndOfInput),
                };
                match pair.code {
                    92 => {
                        path_type = Some(enum_from_number!(
                            BoundaryPathType,
                            Default,
                            from_i16,
                            pair.assert_i16()?
                        ));
                    }
                    97 => {
                        source_boundary_objects_count = pair.assert_i32()?;
                        if boundary_data_read {
                            *loop_count -= 1;
                            break;
                        }
                    }
                    330 => {
                        if source_boundary_objects_count > 0 {
                            //TODO
                        }
                        if boundary_data_read {
                            *loop_count -= 1;
                            break;
                        }
                    }
                    93 => {
                        if path_type.is_some() && !boundary_data_read {
                            iter.put_back(Ok(pair));
                            boundary_data_read =
                                Self::read_edge_boundary(hatch, path_type.unwrap(), iter)?;
                        } else {
                            return Err(DxfError::UnexpectedEndOfInput);
                        }
                    }
                    72 => {
                        if path_type.is_some() && !boundary_data_read {
                            iter.put_back(Ok(pair));
                            boundary_data_read =
                                Self::read_polyline_boundary(hatch, path_type.unwrap(), iter)?;
                        } else {
                            return Err(DxfError::UnexpectedEndOfInput);
                        }
                    }
                    _ => {
                        let code = pair.code;
                        iter.put_back(Ok(pair));
                        if boundary_data_read || code == 0 {
                            return Ok(());
                        } else {
                            return Err(DxfError::UnexpectedCode(code, 0));
                        }
                    }
                }
            }
        }
        Ok(())
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
        let mut polyline_vertices_count: i32 = 0;
        loop {
            let pair = next_pair!(parser);
            match pair.code {
                93 => {
                    // Codigo 93
                    polyline_vertices_count = pair.assert_i32()?;
                }
                72 => {
                    // Codigo 72
                    // let has_bulge = pair.assert_i16()? != 0;
                }
                73 => {
                    // Codigo 73
                    polyline_data.is_closed = pair.assert_i16()? != 0;
                }
                10 | 20 => {
                    parser.put_back(Ok(pair));
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
        loop_count: &mut i32,
        iter: &mut CodePairPutBack,
    ) -> DxfResult<()> {
        let mut pattern_lines: Vec<Self> = Vec::new();
        while *loop_count > 0 {
            let mut pattern_line = Self::default();
            let mut num_dash_items: i32 = 0;
            let mut dash_items_read: i32 = 0;
            let mut mandatory_fields_read: i16 = 0;
            loop {
                let pair = match iter.next() {
                    Some(Ok(pair)) => pair,
                    Some(Err(e)) => return Err(e),
                    None => return Err(DxfError::UnexpectedEndOfInput),
                };
                match pair.code {
                    53 => {
                        pattern_line.angle = pair.assert_f64()?;
                        mandatory_fields_read += 1;
                    }
                    43 => {
                        pattern_line.base_point_x = pair.assert_f64()?;
                        mandatory_fields_read += 1;
                    }
                    44 => {
                        pattern_line.base_point_y = pair.assert_f64()?;
                        mandatory_fields_read += 1;
                    }
                    45 => {
                        pattern_line.offset_x = pair.assert_f64()?;
                        mandatory_fields_read += 1;
                    }
                    46 => {
                        pattern_line.offset_y = pair.assert_f64()?;
                        mandatory_fields_read += 1;
                    }
                    79 => {
                        num_dash_items = pair.assert_i32()?;
                        if num_dash_items <= 0 {
                            break;
                        }
                        mandatory_fields_read += 1;
                    }
                    49 => {
                        if dash_items_read < num_dash_items {
                            pattern_line.dash_lengths.push(pair.assert_f64()?);
                            dash_items_read += 1;
                        }
                        if dash_items_read == num_dash_items {
                            break;
                        }
                    }
                    _ => {
                        iter.put_back(Ok(pair));
                        break;
                    }
                }
            }
            if mandatory_fields_read < 6 {
                return Err(DxfError::UnexpectedEndOfInput);
            }
            *loop_count -= 1;
            pattern_lines.push(pattern_line);
        }
        hatch.pattern_line_data = pattern_lines;
        Ok(())
    }
}
