use traits::default_generic_type_params::{
    defaulted::Point,
    overridden::{Meters, MiliMeters},
};

#[test]
fn default_generic_type_param_default() {
    let (point1, point2) = (Point { x: 2, y: 3 }, Point { x: 4, y: 5 });

    assert_eq!(point1 + point2, Point { x: 6, y: 8 })
}

#[test]
fn default_generic_type_param_overwritten() {
    let (value_meters, value_milimeters) = (Meters(4), MiliMeters(4000));

    assert_eq!(value_milimeters + value_meters, MiliMeters(8000));
}
