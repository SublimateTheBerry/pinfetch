use os_info;
use sys_info;
use whoami;
use colored::Colorize;
use std::env;
use std::process::Command;

fn main() {
    let lang = env::var("LANG").unwrap_or_else(|_| "en_US.UTF-8".to_string());
    let language = match lang.split('.').next().unwrap_or("en_US") {
        "ru_RU" => Language::Russian,
        "de_DE" => Language::German,
        "fr_FR" => Language::French,
        "es_ES" => Language::Spanish,
        _ => Language::English,
    };

    let username = whoami::username();
    let hostname = whoami::fallible::hostname().unwrap_or_else(|_| "Unknown".to_string());
    let os_info = os_info::get();
    let os_type = os_info.os_type().to_string();
    let os_version = os_info.version().to_string();
    let cpu_arch = Command::new("uname").arg("-m").output()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .unwrap_or_else(|_| "Unknown".to_string());
    let cpu_cores = sys_info::cpu_num().map(|num| num.to_string()).unwrap_or_else(|_| "Unknown".to_string());
    let cpu_load = sys_info::loadavg().map(|load| {
        let cores = cpu_cores.parse::<f64>().unwrap_or(1.0);
        format!("{:.2}%", load.one * 100.0 / cores)
    }).unwrap_or_else(|_| "Unknown".to_string());
    let total_memory = sys_info::mem_info().map(|mem| mem.total / 1024).unwrap_or(0);
    let free_memory = sys_info::mem_info().map(|mem| mem.free / 1024).unwrap_or(0);
    let used_memory = total_memory - free_memory;
    let memory_usage = if total_memory > 0 {
        format!("{} MB / {} MB ({:.2}%)", used_memory, total_memory, (used_memory as f64 / total_memory as f64) * 100.0)
    } else {
        "Unknown".to_string()
    };
    let kernel_version = Command::new("uname").arg("-r").output()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .unwrap_or_else(|_| "Unknown".to_string());
    let screen_resolution = Command::new("xrandr").arg("--current").arg("--query").output()
        .map(|output| {
            let output_str = String::from_utf8_lossy(&output.stdout);
            output_str.lines()
                .find(|line| line.contains(" connected ") && !line.contains("disconnected"))
                .and_then(|line| line.split_whitespace().nth(2))
                .unwrap_or("Unknown")
                .to_string()
        })
        .unwrap_or_else(|_| "Unknown".to_string());
    let disk_space = Command::new("df").arg("-h").arg("/").output()
        .map(|output| {
            let output_str = String::from_utf8_lossy(&output.stdout);
            output_str.lines().nth(1)
                .and_then(|line| {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 5 { Some(format!("{} / {}", parts[2], parts[1])) } else { None }
                })
                .unwrap_or("Unknown".to_string())
        })
        .unwrap_or_else(|_| "Unknown".to_string());
    let uptime = Command::new("uptime").output()
        .map(|output| {
            let output_str = String::from_utf8_lossy(&output.stdout);
            output_str.split("up").nth(1)
                .and_then(|uptime_part| uptime_part.split(',').next())
                .unwrap_or("Unknown")
                .trim()
                .to_string()
        })
        .unwrap_or_else(|_| "Unknown".to_string());

    match language {
        Language::English => print_system_info(
            &username, &hostname, &os_type, &os_version, &kernel_version, &cpu_arch, &cpu_cores,
            &cpu_load, &memory_usage, &screen_resolution, &disk_space, &uptime,
            "OS", "Kernel", "Arch", "CPU Cores", "CPU Load", "Memory", "Resolution", "Disk Space", "Uptime",
        ),
        Language::Russian => print_system_info(
            &username, &hostname, &os_type, &os_version, &kernel_version, &cpu_arch, &cpu_cores,
            &cpu_load, &memory_usage, &screen_resolution, &disk_space, &uptime,
            "ОС", "Ядро", "Архитектура", "Ядра CPU", "Загрузка CPU", "Память", "Разрешение", "Дисковое пространство", "Время работы",
        ),
        Language::German => print_system_info(
            &username, &hostname, &os_type, &os_version, &kernel_version, &cpu_arch, &cpu_cores,
            &cpu_load, &memory_usage, &screen_resolution, &disk_space, &uptime,
            "Betriebssystem", "Kernel", "Architektur", "CPU-Kerne", "CPU-Auslastung", "Speicher", "Auflösung", "Festplattenspeicher", "Systemlaufzeit",
        ),
        Language::French => print_system_info(
            &username, &hostname, &os_type, &os_version, &kernel_version, &cpu_arch, &cpu_cores,
            &cpu_load, &memory_usage, &screen_resolution, &disk_space, &uptime,
            "Système d'exploitation", "Noyau", "Architecture", "Cœurs CPU", "Charge CPU", "Mémoire", "Résolution", "Espace disque", "Temps de fonctionnement",
        ),
        Language::Spanish => print_system_info(
            &username, &hostname, &os_type, &os_version, &kernel_version, &cpu_arch, &cpu_cores,
            &cpu_load, &memory_usage, &screen_resolution, &disk_space, &uptime,
            "Sistema operativo", "Kernel", "Arquitectura", "Núcleos de CPU", "Carga de CPU", "Memoria", "Resolución", "Espacio en disco", "Tiempo de actividad",
        ),
    }
}

enum Language {
    English,
    Russian,
    German,
    French,
    Spanish,
}

fn print_system_info(
    username: &str, hostname: &str, os_type: &str, os_version: &str, kernel_version: &str, cpu_arch: &str, cpu_cores: &str,
    cpu_load: &str, memory_usage: &str, screen_resolution: &str, disk_space: &str, uptime: &str,
    os_label: &str, kernel_label: &str, arch_label: &str, cores_label: &str, load_label: &str,
    memory_label: &str, resolution_label: &str, disk_label: &str, uptime_label: &str,
) {
    println!();
    println!("{}@{}", username.bright_green(), hostname.bright_yellow());
    println!("{}: {} {}", os_label.bright_cyan(), os_type, os_version);
    println!("{}: {}", kernel_label.bright_cyan(), kernel_version);
    println!("{}: {}", arch_label.bright_cyan(), cpu_arch);
    println!("{}: {}", cores_label.bright_cyan(), cpu_cores);
    println!("{}: {}", load_label.bright_cyan(), cpu_load);
    println!("{}: {}", memory_label.bright_cyan(), memory_usage);
    println!("{}: {}", resolution_label.bright_cyan(), screen_resolution);
    println!("{}: {}", disk_label.bright_cyan(), disk_space);
    println!("{}: {}", uptime_label.bright_cyan(), uptime);
    println!();
}
