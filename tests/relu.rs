use std::collections::HashMap;
// Indicates a f32 overflow in an intermediate Collatz value

// Args Management
async fn run() {
    let steps = execute_gpu().await.unwrap();

    println!("steps[0..5]: {:#?}", &steps[0..5]);
    #[cfg(target_arch = "wasm32")]
    log::info!("steps[0..5]: {:#?}", &steps[0..5]);
}

// Hardware management
async fn execute_gpu() -> Option<Vec<f32>> {
    // USER INPUT

    let n: usize = 512 * 512 * 128;
    let mut input_data = HashMap::new();
    let data = vec![-1.0f32; n];
    let dims = vec![n as i32];
    input_data.insert("x", (data.as_slice(), dims.as_slice()));

    let session = wonnx::Session::new("tests/single_relu.onnx").await.unwrap();

    session.run(input_data).await
}

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        env_logger::init();
        pollster::block_on(run());
    }
    #[cfg(target_arch = "wasm32")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init().expect("could not initialize logger");
        wasm_bindgen_futures::spawn_local(run());
    }
}
