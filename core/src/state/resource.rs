#![allow(dead_code)]
pub struct Image {
    pub width: usize,
    pub height: usize,
    pub data: Vec<u32>,
}

pub struct ResourceId(u32);

pub struct ResourceManager {
    //pub images: HashMap<String, Image>,
    pub stylesheets: Vec<String>, // Stylesheets refer to a fiel path
    pub themes: Vec<String>,      // Themes are the string content stylesheets
    pub images: Vec<Image>,
}

impl ResourceManager {
    pub fn new() -> Self {
        ResourceManager {
            //images: HashMap::new(),
            stylesheets: Vec::new(),
            themes: Vec::new(),
            images: Vec::new(),
        }
    }

    // TODO
    pub(crate) fn add_image(&mut self, _name: &str, _path: &str) {}

    pub(crate) fn add_font(&mut self, _name: &str, _path: &str) {}
    // pub fn add_stylesheet(&mut self, path: String) -> Result<(), std::io::Error> {

    //     let style_string = std::fs::read_to_string(path.clone())?;
    //     self.stylesheets.push(path);

    //     Ok(())
    // }

    // pub fn load_image(&mut self, name: &str, path: &str) {
    //     let img = image::open(path).expect(&format!("failed to load image: {}", path));
    //     let mut raw_data: Vec<u32> = vec![0; img.to_rgba().into_raw().len() / 4];
    //     LittleEndian::read_u32_into(img.to_rgba().into_raw().as_ref(), &mut raw_data);
    //     let image = Image {
    //         width: img.width() as usize,
    //         height: img.height() as usize,
    //         data: raw_data,
    //     };
    //     self.images.insert(name.to_string(), image);
    // }
}
