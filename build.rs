fn main() {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("doron.ico"); // ruta a tu ícono
        res.compile().unwrap();
    }
}
