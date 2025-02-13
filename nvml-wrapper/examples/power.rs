use std::time::{Duration, Instant};

fn main() -> std::io::Result<()> {
    let x = nvml_wrapper::Nvml::init().map_err(std::io::Error::other)?;
    let dev = x.device_by_index(0).unwrap();
    let mut prev = Instant::now();
    let mut prev_j = dev.total_energy_consumption().unwrap();
    loop {
        // NVML only returns a new sample (where now_j != prev_j) roughly every 100ms.
        // However, our sampling rate for this function _heavily_ affects the underlying
        // value that is returned, even across processes!  If we sample it *more often*
        // it starts incrementing a **lot slower** to the point where it's no longer accurate
        // (not even close, really) to the returned power_usage() value.
        // std::thread::sleep(Duration::from_millis(100));
        let now_j = dev.total_energy_consumption().unwrap();
        if now_j != prev_j {
            let now = Instant::now();
            let d = now - prev;
            let d_j = now_j - prev_j;
            println!("{d:?}: {d_j}mJ ({}mW)", dev.power_usage().unwrap());

            prev_j = now_j;
            prev = now;
        }
    }

    Ok(())
}
