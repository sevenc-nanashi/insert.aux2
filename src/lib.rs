use aviutl2::{anyhow, config::translate as tr, tracing};

static GLOBAL_EDIT_HANDLE: aviutl2::generic::GlobalEditHandle =
    aviutl2::generic::GlobalEditHandle::new();

#[aviutl2::plugin(GenericPlugin)]
struct InsertAux2;

impl aviutl2::generic::GenericPlugin for InsertAux2 {
    fn new(_info: aviutl2::AviUtl2Info) -> aviutl2::AnyResult<Self> {
        aviutl2::tracing_subscriber::fmt()
            .with_max_level(if cfg!(debug_assertions) {
                tracing::Level::DEBUG
            } else {
                tracing::Level::INFO
            })
            .event_format(aviutl2::logger::AviUtl2Formatter)
            .with_writer(aviutl2::logger::AviUtl2LogWriter)
            .init();
        Ok(Self)
    }

    fn plugin_info(&self) -> aviutl2::generic::GenericPluginTable {
        aviutl2::generic::GenericPluginTable {
            name: "insert.aux2".to_string(),
            information: format!(
                "Insert Shortcut / v{} / https://github.com/sevenc-nanashi/insert.aux2",
                env!("CARGO_PKG_VERSION")
            ),
        }
    }

    fn register(&mut self, registry: &mut aviutl2::generic::HostAppHandle) {
        GLOBAL_EDIT_HANDLE.init(registry.create_edit_handle());
        registry.register_menus::<Self>();
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
        tracing::info!("File selected: {}", path);

        GLOBAL_EDIT_HANDLE.call_edit_section(|edit| {
            edit.create_object_from_media_file(path, edit.info.layer, edit.info.frame, None)?;

            anyhow::Ok(())
        })??;

        Ok(())
    }
}

aviutl2::register_generic_plugin!(InsertAux2);
