use aviutl2::{anyhow, log, config::translate as tr};

static GLOBAL_EDIT_HANDLE: aviutl2::generic::GlobalEditHandle =
    aviutl2::generic::GlobalEditHandle::new();

#[aviutl2::plugin(GenericPlugin)]
struct InsertAux2;

impl aviutl2::generic::GenericPlugin for InsertAux2 {
    fn new(_info: aviutl2::AviUtl2Info) -> aviutl2::AnyResult<Self> {
        aviutl2::logger::LogBuilder::new()
            .filter_level(if cfg!(debug_assertions) {
                aviutl2::logger::LevelFilter::Debug
            } else {
                aviutl2::logger::LevelFilter::Info
            })
            .init();
        Ok(Self)
    }

    fn register(&mut self, registry: &mut aviutl2::generic::HostAppHandle) {
        GLOBAL_EDIT_HANDLE.init(registry.create_edit_handle());
        registry.register_menus::<Self>();
        registry.set_plugin_information(&format!(
            "Insert Shortcut / v{} / https://github.com/sevenc-nanashi/insert.aux2",
            env!("CARGO_PKG_VERSION")
        ));
    }
}

#[aviutl2::generic::menus]
impl InsertAux2 {
    #[edit(name = "insert.aux2\\メディアを挿入")]
    fn insert(&mut self) -> anyhow::Result<()> {
        let file = native_dialog::FileDialogBuilder::default()
            .set_title(tr("挿入するメディアファイルを選択"))
            .set_owner(&unsafe { GLOBAL_EDIT_HANDLE.get_host_app_window() }.unwrap())
            .open_single_file()
            .show()?;

        let Some(path) = file else {
            return Ok(());
        };

        let path = path.to_str().unwrap();
        log::info!("File selected: {}", path);

        GLOBAL_EDIT_HANDLE.call_edit_section(|edit| {
            edit.create_object_from_media_file(path, edit.info.layer, edit.info.frame, None)?;

            anyhow::Ok(())
        })??;

        Ok(())
    }
}

aviutl2::register_generic_plugin!(InsertAux2);
