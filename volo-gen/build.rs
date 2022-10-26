
fn main() {
    // volo_build::ConfigBuilder::default()
    //     .write().unwrap();

    volo_build::ConfigBuilder::new("volo.yml".into())    
        .write().unwrap();
}
