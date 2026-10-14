//! The chat box text, from a status sample of pcsc-rs (the JSON it sends to PC Status).

use serde_json::Value;

/// VRChat cuts the chat box at this many characters.
pub const CHATBOX_LIMIT: usize = 144;

/// The figures the chat box shows, read from a sample. None where the sample has none
/// (no GPU, nothing measured yet).
#[derive(Debug, Default, PartialEq)]
pub struct Figures {
    /// Percent, averaged over the cores
    pub cpu: Option<f64>,
    /// Used and total bytes
    pub ram: Option<(u64, u64)>,
    pub gpu: Option<Gpu>,
    /// Bytes read and written per second, over every disk
    pub disk: Option<(u64, u64)>,
}

#[derive(Debug, PartialEq)]
pub struct Gpu {
    pub usage: f64,
    /// Used and total bytes; None for GPUs without memory of their own
    pub vram: Option<(u64, u64)>,
}

const MIB: u64 = 1024 * 1024;

impl Figures {
    // Core counts and byte figures are far below where f64 loses precision
    #[allow(clippy::cast_precision_loss)]
    pub fn read(status: &Value) -> Self {
        let number = |v: &Value| v.as_f64();
        let cpu = status["cpu"]["cpus"].as_array().and_then(|cores| {
            let usages: Vec<f64> = cores.iter().filter_map(|c| number(&c["cpu"])).collect();
            (!usages.is_empty()).then(|| usages.iter().sum::<f64>() / usages.len() as f64)
        });
        let ram = memory(&status["ram"], 1);
        // The GPU with memory of its own (the dedicated one beside an integrated one), else the first
        let gpus = status["gpus"]
            .as_array()
            .map(Vec::as_slice)
            .unwrap_or_default();
        let gpu = gpus
            .iter()
            .find(|g| g["memory"]["total"].as_u64().unwrap_or(0) > 0)
            .or_else(|| gpus.first())
            .map(|g| Gpu {
                usage: number(&g["usage"]).unwrap_or(0.0),
                vram: memory(&g["memory"], MIB),
            });
        let disk = status["storages"]
            .as_array()
            .filter(|s| !s.is_empty())
            .map(|storages| {
                storages.iter().fold((0, 0), |(r, w), s| {
                    (
                        r + s["read"].as_u64().unwrap_or(0),
                        w + s["written"].as_u64().unwrap_or(0),
                    )
                })
            });
        Self {
            cpu,
            ram,
            gpu,
            disk,
        }
    }
}

/// (used, total) in bytes from `{free, total}` in `unit`s; None when the total is unknown.
fn memory(value: &Value, unit: u64) -> Option<(u64, u64)> {
    let total = value["total"].as_u64()?;
    let free = value["free"].as_u64().unwrap_or(0);
    (total > 0).then(|| (total.saturating_sub(free) * unit, total * unit))
}

/// `8.4 GB`, `467 MB`, `0 B`.
#[allow(clippy::cast_precision_loss)]
pub fn bytes(n: u64) -> String {
    const UNITS: [&str; 6] = ["B", "KB", "MB", "GB", "TB", "PB"];
    let mut value = n as f64;
    let mut unit = 0;
    while value >= 1024.0 && unit < UNITS.len() - 1 {
        value /= 1024.0;
        unit += 1;
    }
    if unit == 0 || value >= 100.0 {
        format!("{value:.0} {}", UNITS[unit])
    } else {
        format!("{value:.1} {}", UNITS[unit])
    }
}

fn percent(value: f64) -> String {
    format!("{}%", value.clamp(0.0, 100.0).floor())
}

#[allow(clippy::cast_precision_loss)]
fn used_percent((used, total): (u64, u64)) -> f64 {
    used as f64 / total as f64 * 100.0
}

/// The placeholders a format can use, and their values (`--` where there is none).
fn placeholders(figures: &Figures, text: &str) -> Vec<(&'static str, String)> {
    let none = || "--".to_owned();
    let vram = figures.gpu.as_ref().and_then(|g| g.vram);
    vec![
        ("cpu", figures.cpu.map_or_else(none, percent)),
        (
            "ram",
            figures.ram.map(used_percent).map_or_else(none, percent),
        ),
        ("ram_used", figures.ram.map_or_else(none, |(u, _)| bytes(u))),
        (
            "ram_total",
            figures.ram.map_or_else(none, |(_, t)| bytes(t)),
        ),
        (
            "gpu",
            figures.gpu.as_ref().map_or_else(none, |g| percent(g.usage)),
        ),
        ("vram", vram.map(used_percent).map_or_else(none, percent)),
        ("vram_used", vram.map_or_else(none, |(u, _)| bytes(u))),
        ("vram_total", vram.map_or_else(none, |(_, t)| bytes(t))),
        (
            "disk_read",
            figures.disk.map_or_else(none, |(r, _)| bytes(r)),
        ),
        (
            "disk_write",
            figures.disk.map_or_else(none, |(_, w)| bytes(w)),
        ),
        ("text", text.to_owned()),
    ]
}

/// The names `--format` may use, for `--help`.
pub const PLACEHOLDERS: &str = "{cpu} {ram} {ram_used} {ram_total} {gpu} {vram} {vram_used} {vram_total} {disk_read} {disk_write} {text}";

/// The chat box text: `format` with its placeholders filled in (and `\n` as a line break),
/// or, without one, the usual line, leaving out what this PC has not got:
/// `CPU: 12% | RAM: 55% (8.4 GB / 15.3 GB) | GPU: 30% | VRAM: 11% (467 MB / 4.0 GB) | Disk: R 1.2 MB/s W 300 KB/s | [text]`.
/// Cut at the chat box's limit.
pub fn render(figures: &Figures, text: &str, format: Option<&str>) -> String {
    let message = if let Some(format) = format {
        fill(format, figures, text)
    } else {
        usual_line(figures, text)
    };
    message.chars().take(CHATBOX_LIMIT).collect()
}

fn fill(format: &str, figures: &Figures, text: &str) -> String {
    let mut message = format.replace("\\n", "\n");
    for (name, value) in placeholders(figures, text) {
        message = message.replace(&format!("{{{name}}}"), &value);
    }
    message
}

fn usual_line(figures: &Figures, text: &str) -> String {
    let p = placeholders(figures, text);
    let get = |name: &str| {
        p.iter()
            .find(|(n, _)| *n == name)
            .map_or("--", |(_, v)| v.as_str())
    };
    let mut parts = vec![
        format!("CPU: {}", get("cpu")),
        format!(
            "RAM: {} ({} / {})",
            get("ram"),
            get("ram_used"),
            get("ram_total")
        ),
    ];
    if let Some(gpu) = &figures.gpu {
        parts.push(format!("GPU: {}", get("gpu")));
        if gpu.vram.is_some() {
            parts.push(format!(
                "VRAM: {} ({} / {})",
                get("vram"),
                get("vram_used"),
                get("vram_total")
            ));
        }
    }
    if figures.disk.is_some() {
        parts.push(format!(
            "Disk: R {}/s W {}/s",
            get("disk_read"),
            get("disk_write")
        ));
    }
    if !text.is_empty() {
        parts.push(format!("[{text}]"));
    }
    parts.join(" | ")
}

#[cfg(test)]
mod tests {
    use super::{Figures, Gpu, bytes, render};
    use serde_json::json;

    fn sample() -> serde_json::Value {
        json!({
            "cpu": { "model": "cpu", "cpus": [{ "cpu": 10.0 }, { "cpu": 20.5 }] },
            "ram": { "free": 8_u64 << 30, "total": 16_u64 << 30 },
            "gpus": [
                { "name": "Intel UHD Graphics", "usage": 5.0, "memory": { "free": 0, "total": 0 } },
                { "name": "NVIDIA GeForce RTX 3050 Ti Laptop GPU", "usage": 30.0, "memory": { "free": 3584, "total": 4096 } }
            ],
            "storages": [
                { "name": "/", "free": 1, "total": 2, "read": 1_200_000, "written": 300_000 },
                { "name": "/home", "free": 1, "total": 2, "read": 100_000, "written": 0 }
            ]
        })
    }

    #[test]
    fn reads_a_sample() {
        let f = Figures::read(&sample());
        assert_eq!(f.cpu, Some(15.25));
        assert_eq!(f.ram, Some((8 << 30, 16 << 30)));
        // The GPU with its own memory, not the integrated one listed first
        assert_eq!(
            f.gpu,
            Some(Gpu {
                usage: 30.0,
                vram: Some((512 << 20, 4096 << 20))
            })
        );
        assert_eq!(f.disk, Some((1_300_000, 300_000)));
        assert_eq!(Figures::read(&json!({})), Figures::default());
    }

    #[test]
    fn renders_the_line() {
        let f = Figures::read(&sample());
        assert_eq!(
            render(&f, "hello", None),
            "CPU: 15% | RAM: 50% (8.0 GB / 16.0 GB) | GPU: 30% | VRAM: 12% (512 MB / 4.0 GB) | Disk: R 1.2 MB/s W 293 KB/s | [hello]"
        );
        // No GPU, no disks, no text: left out
        let bare = Figures::read(
            &json!({ "cpu": { "cpus": [{ "cpu": 50.0 }] }, "ram": { "free": 1, "total": 2 } }),
        );
        assert_eq!(render(&bare, "", None), "CPU: 50% | RAM: 50% (1 B / 2 B)");
    }

    #[test]
    fn renders_a_format() {
        let f = Figures::read(&sample());
        assert_eq!(
            render(&f, "", Some("CPU {cpu}\\nGPU {gpu} / {vram_total}")),
            "CPU 15%\nGPU 30% / 4.0 GB"
        );
        assert_eq!(
            render(&Figures::default(), "x", Some("{gpu} {text}")),
            "-- x"
        );
        // Cut at the chat box's limit
        assert_eq!(
            render(&f, &"あ".repeat(200), Some("{text}"))
                .chars()
                .count(),
            144
        );
    }

    #[test]
    fn formats_bytes() {
        assert_eq!(bytes(0), "0 B");
        assert_eq!(bytes(1023), "1023 B");
        assert_eq!(bytes(1536), "1.5 KB");
        assert_eq!(bytes(467 << 20), "467 MB");
        assert_eq!(bytes(9 << 30 | 400 << 20), "9.4 GB");
    }
}
