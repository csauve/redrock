use crate::game::GameState;
use crate::render::Window;
use wgpu;

pub struct Renderer {
    instance: wgpu::Instance,
    surface: wgpu::Surface,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
}

impl Renderer {
    pub async fn new(window: &Window) -> Renderer {
        let backend = wgpu::util::backend_bits_from_env().unwrap_or(wgpu::Backends::PRIMARY);
        let instance = wgpu::Instance::new(backend);
        let surface = unsafe {instance.create_surface(&window.window)};
        let adapter = wgpu::util::initialize_adapter_from_env_or_default(&instance, backend)
            .await
            .expect("Failed to find adapter");

        println!("Found adapter {}", adapter.get_info().name);
        // adapter.limits();
        // adapter.features();

        let descriptor = wgpu::DeviceDescriptor {
            label: None,
            features: wgpu::Features::empty(),
            limits: wgpu::Limits::downlevel_defaults(),
        };

        let (device, queue) = adapter.request_device(&descriptor, None)
            .await
            .expect("Failed to request device");

        Renderer {
            instance,
            surface,
            adapter,
            device,
            queue,
        }
    }

    pub fn render(&self, game_state: &GameState) {
        // self.queue.
        // println!("render");
        // for (_id, item) in game_state.objects.iter() {
        //     // dbg!(item);
        // }
    }
}
