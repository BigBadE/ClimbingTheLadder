use core::rendering::renderable::Renderable;

pub struct UIWindow {

}

impl Renderable for UIWindow {
    fn render<'a>(&self) -> &'a [&core::rendering::mesh::Mesh] {
        todo!()
    }
}