use std::cmp::PartialOrd;

// Using generics in structs: 
struct Point<T> {
    x: T,
    y: T
}

// We can define methods on structs using generics as well 
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// We can also define methods using concrete types that only apply to 
// the type defined
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
 }

 // You can also mixup generics by using passed in generics in the method
 // signature and the ones defined in the struct
 struct PointV2<X1, Y1> {
    x: X1,
    y: Y1,
 }

 // The method returns the X1 from the PointV2 struct and the Y2 that's passed
 // in to the method
 impl<X1, Y1> PointV2<X1, Y1> {
     fn mixup<X2, Y2>(self, other: PointV2<X2, Y2>) -> PointV2<X1, Y2> {
         PointV2 {
             x: self.x,
             y: other.y,
         }
     }
 }

fn main() {

    // Generics for structs must have the same type 
    // let wont_work = Point { x: 5, y: 4.0 }
    let will_work = Point { x: 5, y: 4 };

    // Using multiple types
    let p1 = PointV2 { x: 5, y: 10.4 };
    let p2 = PointV2 { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    

}

// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list {
//         if item > largest {
//             largest = item
//         }
//     }

//     largest
// }
