fn main() {
    println!("cargo:rerun-if-changed=assets/orgue_icone.ico");

    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        let mut resource = winresource::WindowsResource::new();
        resource.set_icon("assets/orgue_icone.ico");
        resource
            .compile()
            .expect("échec de l'intégration de l'icône Windows");
    }
}
