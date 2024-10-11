/// shape abstraction
pub trait CanCanculateArea {
    fn calculate_area(&self) -> f64;
}

pub trait IsDrawable {
    type Level;
    fn draw(&self) -> String;
}

pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub struct Rectangle {
    pub start: Point,
    pub width: f64,
    pub height: f64,
}

impl CanCanculateArea for Rectangle {
    fn calculate_area(&self) -> f64 {
        self.height * self.width
    }
}

impl IsDrawable for Rectangle {
    type Level = u32;
    fn draw(&self) -> String {
        "[Rectangle]".to_string()
    }
}

pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

impl CanCanculateArea for Circle {
    fn calculate_area(&self) -> f64 {
        self.radius
    }
}

impl IsDrawable for Circle {
    type Level = u32;
    fn draw(&self) -> String {
        "(Circle)".to_string()
    }
}

pub fn generic_calculate_area<T: CanCanculateArea>(shape: T) -> f64 {
    shape.calculate_area()
}

pub fn draw_static<T: IsDrawable>(s: T) -> String {
    format!("shared,{}", s.draw())
}

/// &dyn IsDrawable
/// is fat/wide pointer
/// it stores two things
/// - pointer to the data (instance of a concrete implementation)
/// - pointer to a vtable:
///     - contains a referenceo to the concrete implementation of the trait method
///     - and other info about the concrete type
///
/// # limitations
///
/// ## can not use it with regular traits
/// ```rust
/// &(dyn IsDrawable  + CanCanculateArea) // does not compile
/// ```
/// because the rust compiler, has now to store more deata about the
/// concrete type
/// - we need a reference to the vtable of the IsDrawable with the concrete type
///     and a reference to the vtable of the CanCanculateArea with concrete type
/// - which means that now we need even a wider pointer :):
///
///  --------------------------------
///  | data  | vtable 1  | vtable 2 |
///  -------------------------------
///
/// note:  it can be combined with marker traits (like sized trait)
///
///  ## can not be used with associated type
///
/// associated types are like generics, which means that concrete type can implement
/// the same trait multiple times with different associated items
///
///
/// for this reason, we have specify the item type, when using trait objects
///
/// ## can now have associated methods (methods that does not take self)
/// it does not make sense. we need the concrete type to be able to call such a method
/// it can not be called on an instance of the type (like static methods in java)
/// we execlude a method from the vtable by specifying that the method
/// is only reauired when the concrete type is sized
/// Since the trait object (which acts like an implementation of the trait) is not sized
/// (only the pointer that represent it is sized) methods where self is sized will
/// not be added to the trait object vtable
///
pub fn draw_dynamic<T>(s: &dyn IsDrawable<Level = T>) -> String {
    format!("shared,{}", s.draw())
}

/// &dyn T is a wide pointer
///
/// for the concrete Rectangle type, it looks something like this
/// - pointer to the type T itself
/// - pointer to the vtable which map the trait function to its concrete
///     implementation for this concrete type
///     looks something like this
///     struct vtable {
///         calculate_area: // trait method
///             &<Rectangle as CanCanculateArea>::calculate_area // line  18
///     }
/// i think these vtables of the types get created once we use a trait
/// as a trait object
/// because otherwise we do not need them
///
/// note: it is a table because a type can implement multiple traits,
/// and it needs to keep track of all concrete implementation of all trait it implements
impl CanCanculateArea for Vec<&dyn CanCanculateArea> {
    fn calculate_area(&self) -> f64 {
        let mut total = 0.0;
        for shape in self {
            total += shape.calculate_area();
        }
        total
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_circle() {
        let c = Circle {
            center: Point { x: 1.0, y: 1.0 },
            radius: 2.0,
        };

        assert_eq!(c.calculate_area(), 2.0);
    }

    #[test]
    fn test_rectangle() {
        let r = Rectangle {
            start: Point { x: 1.0, y: 1.0 },
            width: 2.0,
            height: 2.0,
        };

        assert_eq!(r.calculate_area(), 4.0);
    }

    #[test]
    fn test_generic_can_calculate_area() {
        let r = Rectangle {
            start: Point { x: 1.0, y: 1.0 },
            width: 2.0,
            height: 2.0,
        };
        assert_eq!(generic_calculate_area(r), 4.0);
    }

    #[test]
    fn calculate_area_for_list_of_shapes() {
        // construt this vector from user input
        //
        // we do not know, what the vector will look like
        //
        // because of that, the compiler has to generate the vtable
        // for all types that implement the trait

        let r = Rectangle {
            start: Point { x: 1.0, y: 1.0 },
            width: 2.0,
            height: 2.0,
        };
        let r_ref = &r;
        let shapes: Vec<&dyn CanCanculateArea> = vec![
            r_ref,
            &Circle {
                center: Point { x: 1.0, y: 1.0 },
                radius: 2.0,
            },
        ];

        assert_eq!(shapes.calculate_area(), 6.0);
    }

    #[test]
    fn test_draw_static() {
        let r = Rectangle {
            start: Point { x: 1.0, y: 1.0 },
            width: 2.0,
            height: 2.0,
        };
        assert_eq!(draw_static(r), "shared,[Rectangle]");

        let c = Circle {
            center: Point { x: 1.0, y: 1.0 },
            radius: 2.0,
        };
        assert_eq!(draw_static(c), "shared,(Circle)");
    }

    #[test]
    fn test_draw_trait_objects() {
        let user_input = true;
        let res: &dyn IsDrawable<Level = u32> = match user_input {
            true => &Rectangle {
                start: Point { x: 1.0, y: 1.0 },
                width: 2.0,
                height: 2.0,
            },
            false => &Circle {
                center: Point { x: 1.0, y: 1.0 },
                radius: 2.0,
            },
        };

        let res = draw_dynamic(res);
        assert_eq!(res, "shared,[Rectangle]");
    }
}
