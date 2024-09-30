fn main() {
    let mut filesystem = luminol_filesystem::project::FileSystem::new();
    // hardcoding the path for now
    let host = luminol_filesystem::host::FileSystem::new("/mnt/hdd/Git/OSFM-GitHub");

    let mut global_config = luminol_config::global::Config::new();
    let mut config = None;

    let _ = filesystem
        .load_project(host, &mut config, &mut global_config)
        .unwrap();

    let mut config = config.unwrap();

    let mut toasts = luminol_core::Toasts::default();
    let mut data_cache = luminol_core::Data::Unloaded;
    data_cache
        .load(&filesystem, &mut toasts, &mut config)
        .unwrap();

    let mut map = data_cache.get_or_load_map(1, &filesystem, &config);
    let event = map.events.remove(1);
}
