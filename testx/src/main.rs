use llml::EucVecf2;

fn main () {
    let alpha = EucVecf2::new(1., 2.);

    let serialized = serde_json::to_string(&alpha).unwrap();
    println!("serialized = {}", serialized);

    let deserialized: EucVecf2 = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);
}