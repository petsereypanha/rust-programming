fn sum_int(x:i32, y:i32) -> i32 {
    x+y
}
#[derive(Debug)]
struct Circle<T> {
    cx : T,
    cy : T,
    r : T
}

fn generics() {
    let a = 10;
    let b = 20;
    let c = sum_int(a, b);
    println!("{}", c);

    let c1 = Circle {
        cx : 10,
        cy : 20,
        r : 5
    };
    let c = Circle1{
        cx: 10,
        cy: 20,
        r : 5.0
    };
    draw_shape(&c);

}

struct Circle1 {
    cx: i32,
    cy: i32,
    r: f64
}
trait ShapeUtils {
    fn print_shape(&self);
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

// Implementing the ShapeUtils trait on Circle struct
impl ShapeUtils for Circle1{
    fn print_shape(&self) {
        println!("Circle : [c = ({},{}), r = {}]", self.cx, self.cy, self.r);
    }
    fn area(&self) -> f64{
        3.14*self.r*self.r
    }
    fn perimeter(&self) -> f64{
        2.0*3.14*self.r
    }
}
//fn draw_shape(shape: &impl ShapeUtils) {
//    shape.print_shape();
//}

// Trait bound syntax
fn draw_shape<T: ShapeUtils>(shape: &T){
    shape.print_shape();
}