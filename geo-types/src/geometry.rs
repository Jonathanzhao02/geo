use crate::{
    CoordNum, Error, GeometryCollection, Line, LineString, MultiLineString, MultiPoint,
    MultiPolygon, Point, Polygon, Rect, Triangle,
};
use core::any::type_name;
use std::convert::TryFrom;

/// An enum representing any possible geometry type.
///
/// All `Geo` types can be converted to a `Geometry` member using `.into()` (as part of the
/// `std::convert::Into` pattern), and `Geo` types implement the `TryFrom` trait in order to
/// convert _back_ from enum members.
///
/// # Example
///
/// ```
/// use std::convert::TryFrom;
/// use geo_types::{Point, point, Geometry, GeometryCollection};
/// let p = point!(x: 1.0, y: 1.0);
/// let pe: Geometry<f64> = p.into();
/// let pn = Point::try_from(pe).unwrap();
/// ```
///
#[derive(Eq, PartialEq, Clone, Debug, Hash)]
pub enum Geometry<T>
where
    T: CoordNum,
{
    Point(Point<T>),
    Line(Line<T>),
    LineString(LineString<T>),
    Polygon(Polygon<T>),
    MultiPoint(MultiPoint<T>),
    MultiLineString(MultiLineString<T>),
    MultiPolygon(MultiPolygon<T>),
    GeometryCollection(GeometryCollection<T>),
    Rect(Rect<T>),
    Triangle(Triangle<T>),
}

impl<T: CoordNum> From<Point<T>> for Geometry<T> {
    fn from(x: Point<T>) -> Geometry<T> {
        Geometry::Point(x)
    }
}
impl<T: CoordNum> From<Line<T>> for Geometry<T> {
    fn from(x: Line<T>) -> Geometry<T> {
        Geometry::Line(x)
    }
}
impl<T: CoordNum> From<LineString<T>> for Geometry<T> {
    fn from(x: LineString<T>) -> Geometry<T> {
        Geometry::LineString(x)
    }
}
impl<T: CoordNum> From<Polygon<T>> for Geometry<T> {
    fn from(x: Polygon<T>) -> Geometry<T> {
        Geometry::Polygon(x)
    }
}
impl<T: CoordNum> From<MultiPoint<T>> for Geometry<T> {
    fn from(x: MultiPoint<T>) -> Geometry<T> {
        Geometry::MultiPoint(x)
    }
}
impl<T: CoordNum> From<MultiLineString<T>> for Geometry<T> {
    fn from(x: MultiLineString<T>) -> Geometry<T> {
        Geometry::MultiLineString(x)
    }
}
impl<T: CoordNum> From<MultiPolygon<T>> for Geometry<T> {
    fn from(x: MultiPolygon<T>) -> Geometry<T> {
        Geometry::MultiPolygon(x)
    }
}

impl<T: CoordNum> From<Rect<T>> for Geometry<T> {
    fn from(x: Rect<T>) -> Geometry<T> {
        Geometry::Rect(x)
    }
}

impl<T: CoordNum> From<Triangle<T>> for Geometry<T> {
    fn from(x: Triangle<T>) -> Geometry<T> {
        Geometry::Triangle(x)
    }
}

impl<T: CoordNum> Geometry<T> {
    /// If this Geometry is a Point, then return that, else None.
    ///
    /// # Examples
    ///
    /// ```
    /// use geo_types::*;
    /// let g = Geometry::Point(Point::new(0., 0.));
    /// let p2: Point<f32> = g.into_point().unwrap();
    /// assert_eq!(p2, Point::new(0., 0.,));
    /// ```
    #[deprecated(
        note = "Will be removed in an upcoming version. Switch to std::convert::TryInto<Point>"
    )]
    pub fn into_point(self) -> Option<Point<T>> {
        if let Geometry::Point(x) = self {
            Some(x)
        } else {
            None
        }
    }

    /// If this Geometry is a LineString, then return that LineString, else None.
    #[deprecated(
        note = "Will be removed in an upcoming version. Switch to std::convert::TryInto<LineString>"
    )]
    pub fn into_line_string(self) -> Option<LineString<T>> {
        if let Geometry::LineString(x) = self {
            Some(x)
        } else {
            None
        }
    }

    /// If this Geometry is a Line, then return that Line, else None.
    #[deprecated(
        note = "Will be removed in an upcoming version. Switch to std::convert::TryInto<Line>"
    )]
    pub fn into_line(self) -> Option<Line<T>> {
        if let Geometry::Line(x) = self {
            Some(x)
        } else {
            None
        }
    }

    /// If this Geometry is a Polygon, then return that, else None.
    #[deprecated(
        note = "Will be removed in an upcoming version. Switch to std::convert::TryInto<Polygon>"
    )]
    pub fn into_polygon(self) -> Option<Polygon<T>> {
        if let Geometry::Polygon(x) = self {
            Some(x)
        } else {
            None
        }
    }

    /// If this Geometry is a MultiPoint, then return that, else None.
    #[deprecated(
        note = "Will be removed in an upcoming version. Switch to std::convert::TryInto<MultiPoint>"
    )]
    pub fn into_multi_point(self) -> Option<MultiPoint<T>> {
        if let Geometry::MultiPoint(x) = self {
            Some(x)
        } else {
            None
        }
    }

    /// If this Geometry is a MultiLineString, then return that, else None.
    #[deprecated(
        note = "Will be removed in an upcoming version. Switch to std::convert::TryInto<MultiLineString>"
    )]
    pub fn into_multi_line_string(self) -> Option<MultiLineString<T>> {
        if let Geometry::MultiLineString(x) = self {
            Some(x)
        } else {
            None
        }
    }

    /// If this Geometry is a MultiPolygon, then return that, else None.
    #[deprecated(
        note = "Will be removed in an upcoming version. Switch to std::convert::TryInto<MultiPolygon>"
    )]
    pub fn into_multi_polygon(self) -> Option<MultiPolygon<T>> {
        if let Geometry::MultiPolygon(x) = self {
            Some(x)
        } else {
            None
        }
    }
}

#[macro_use]
macro_rules! try_from_geometry_impl {
    ($($type: ident),+) => {
        $(
        /// Convert a Geometry enum into its inner type.
        ///
        /// Fails if the enum case does not match the type you are trying to convert it to.
        impl <T: CoordNum> TryFrom<Geometry<T>> for $type<T> {
            type Error = Error;

            fn try_from(geom: Geometry<T>) -> Result<Self, Self::Error> {
                match geom {
                    Geometry::$type(g) => Ok(g),
                    other => Err(Error::MismatchedGeometry {
                        expected: type_name::<$type<T>>(),
                        found: inner_type_name(other)
                    })
                }
            }
        }
        )+
    }
}

try_from_geometry_impl!(
    Point,
    Line,
    LineString,
    Polygon,
    MultiPoint,
    MultiLineString,
    MultiPolygon,
    Rect,
    Triangle
);

fn inner_type_name<T>(geometry: Geometry<T>) -> &'static str
where
    T: CoordNum,
{
    match geometry {
        Geometry::Point(_) => type_name::<Point<T>>(),
        Geometry::Line(_) => type_name::<Line<T>>(),
        Geometry::LineString(_) => type_name::<LineString<T>>(),
        Geometry::Polygon(_) => type_name::<Polygon<T>>(),
        Geometry::MultiPoint(_) => type_name::<MultiPoint<T>>(),
        Geometry::MultiLineString(_) => type_name::<MultiLineString<T>>(),
        Geometry::MultiPolygon(_) => type_name::<MultiPolygon<T>>(),
        Geometry::GeometryCollection(_) => type_name::<GeometryCollection<T>>(),
        Geometry::Rect(_) => type_name::<Rect<T>>(),
        Geometry::Triangle(_) => type_name::<Triangle<T>>(),
    }
}
