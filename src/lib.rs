use bevy::app::App;
use bevy::asset::AssetPath;
use bevy::asset::io::embedded::EmbeddedAssetRegistry;
pub use bevy_wgsl_rs_macros::*;
use std::path::PathBuf;
use wgsl_rs::Module;

pub fn embed_wgsl_rs_shader_impl<'a>(
    app: &mut App,
    module_path: &str,
    wgsl_module: &Module,
) -> AssetPath<'a> {
    let module_path = module_path.replace("::", "/");
    let module_path = format!("embedded://{module_path}.wgsl");
    let module_path = AssetPath::from_path_buf(module_path.into());
    let shader_source = wgsl_module.wgsl_source().join("\n");
    let embedded = app.world_mut().resource_mut::<EmbeddedAssetRegistry>();
    embedded.insert_asset(
        PathBuf::from(""),
        module_path.path(),
        shader_source.into_bytes(),
    );
    module_path
}

#[macro_export]
macro_rules! embed_wgsl_rs_shader {
    ($app: expr, $module: path) => {
        $crate::embed_wgsl_rs_shader_impl($app, &$module::MODULE_PATH, &$module::WGSL_MODULE)
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::app::{App, TaskPoolPlugin};
    use bevy::asset::io::Reader;
    use bevy::asset::io::embedded::GetAssetServer;
    use bevy::asset::{
        Asset, AssetApp, AssetId, AssetLoader, AssetPlugin, Assets, AsyncReadExt, Handle,
        LoadContext,
    };
    use bevy::diagnostic::DiagnosticsPlugin;
    use bevy::prelude::{BevyError, World};
    use bevy::reflect::TypePath;
    use wgsl_rs::wgsl;

    #[bevy_wgsl]
    mod hello_triangle {
        //! This is a "hello world" shader that shows a triangle with changing
        //! color. Original source is [here](https://google.github.io/tour-of-wgsl/).

        // Only glob-imports are supported, but hey, imports work!
        use wgsl_rs::std::*;

        // Define a uniform in both Rust and WGSL using the uniform! macro.
        uniform!(group(0), binding(0), FRAME: u32);

        #[vertex]
        #[allow(unused, reason = "shader code")]
        pub fn vtx_main(#[builtin(vertex_index)] vertex_index: u32) -> Vec4f {
            const POS: [Vec2f; 3] = [vec2f(0.0, 0.5), vec2f(-0.5, -0.5), vec2f(0.5, -0.5)];

            let position = POS[vertex_index as usize];
            vec4f(position.x, position.y, 0.0, 1.0)
        }

        #[fragment]
        #[allow(unused, reason = "shader code")]
        pub fn frag_main() -> Vec4f {
            vec4f(1.0, sin(f32(get!(FRAME)) / 128.0), 0.0, 1.0)
        }
    }

    #[derive(Asset, TypePath, Debug)]
    pub struct ShaderText(String);

    #[derive(Default, TypePath)]
    struct ShaderTextAssetLoader;

    impl AssetLoader for ShaderTextAssetLoader {
        type Asset = ShaderText;
        type Settings = ();
        type Error = BevyError;

        async fn load(
            &self,
            reader: &mut dyn Reader,
            settings: &Self::Settings,
            load_context: &mut LoadContext<'_>,
        ) -> Result<Self::Asset, Self::Error> {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;

            Ok(ShaderText(String::from_utf8(bytes)?))
        }
    }

    fn get_asset<A: Asset>(world: &World, id: AssetId<A>) -> Option<&A> {
        world.resource::<Assets<A>>().get(id)
    }

    #[tokio::test]
    async fn test_macro() {
        let mut app = App::new();
        app.add_plugins((
            TaskPoolPlugin::default(),
            AssetPlugin::default(),
            DiagnosticsPlugin,
        ));
        app.init_asset::<ShaderText>();
        app.register_asset_loader(ShaderTextAssetLoader);
        let asset_path = embed_wgsl_rs_shader!(&mut app, hello_triangle);
        let shader_text_handle: Handle<ShaderText> = {
            let asset_server = app.get_asset_server();
            asset_server.load(asset_path)
        };
        app.update();

        let shader_text = get_asset(app.world(), shader_text_handle.id());
        assert!(matches!(shader_text, Some(_)));
    }
}
