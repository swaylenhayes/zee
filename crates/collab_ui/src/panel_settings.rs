use gpui::Pixels;
use settings::{DockPanelMode, RegisterSetting, Settings};
use ui::px;
use workspace::dock::DockPosition;

#[derive(Debug, RegisterSetting)]
pub struct CollaborationPanelSettings {
    pub button: bool,
    pub dock: DockPosition,
    pub dock_panel_mode: Option<DockPanelMode>,
    pub default_width: Pixels,
}

impl Settings for CollaborationPanelSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let panel = content.collaboration_panel.as_ref().unwrap();

        Self {
            button: panel.button.unwrap(),
            dock: panel.dock.unwrap().into(),
            dock_panel_mode: panel.dock_panel_mode,
            default_width: panel.default_width.map(px).unwrap(),
        }
    }
}
