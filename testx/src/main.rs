use llml::others::Complx;

fn main () {
    let test = Complx::new(1, 2);
    let angle : Complx<f32> = test.into();
}