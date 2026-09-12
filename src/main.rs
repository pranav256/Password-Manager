mod crypto;
mod generator;
mod vault;

use crypto::DerivedKey;
use eframe::egui::{self, Color32, RichText, Vec2};
use generator::{calculate_entropy, generate_password, strength_rating, GeneratorConfig};
use std::path::{Path, PathBuf};
use vault::{Vault, VaultEntry};

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 620.0])
            .with_min_inner_size([700.0, 500.0])
            .with_title("RustyVault - Secure Password Manager"),
        ..Default::default()
    };

    eframe::run_native(
        "RustyVault",
        options,
        Box::new(|cc| {
            let mut visuals = egui::Visuals::dark();
            visuals.window_rounding = 8.0.into();
            visuals.widgets.noninteractive.rounding = 6.0.into();
            visuals.widgets.inactive.rounding = 6.0.into();
            visuals.widgets.hovered.rounding = 6.0.into();
            visuals.widgets.active.rounding = 6.0.into();
            cc.egui_ctx.set_visuals(visuals);

            Ok(Box::new(RustyVaultApp::new()))
        }),
    )
}

struct RustyVaultApp {
    is_locked: bool,
    vault: Option<Vault>,
    key: Option<DerivedKey>,
    vault_path: PathBuf,
    master_password_input: String,
    confirm_password_input: String,
    is_new_vault: bool,
    unlock_error: Option<String>,

    // Search
    search_query: String,

    // Add / Edit Entry Dialog
    show_entry_dialog: bool,
    editing_entry: Option<VaultEntry>,
    edit_service: String,
    edit_username: String,
    edit_email: String,
    edit_password: String,
    edit_url: String,
    edit_notes: String,
    edit_show_password: bool,

    // Password Generator Modal
    show_generator_modal: bool,
    generator_config: GeneratorConfig,
    generated_password: String,

    // Notifications
    toast_message: Option<(String, std::time::Instant)>,
}

impl RustyVaultApp {
    fn new() -> Self {
        let vault_path = dirs_vault_path();
        let is_new_vault = !vault_path.exists();

        Self {
            is_locked: true,
            vault: None,
            key: None,
            vault_path,
            master_password_input: String::new(),
            confirm_password_input: String::new(),
            is_new_vault,
            unlock_error: None,

            search_query: String::new(),

            show_entry_dialog: false,
            editing_entry: None,
            edit_service: String::new(),
            edit_username: String::new(),
            edit_email: String::new(),
            edit_password: String::new(),
            edit_url: String::new(),
            edit_notes: String::new(),
            edit_show_password: false,

            show_generator_modal: false,
            generator_config: GeneratorConfig::default(),
            generated_password: generate_password(&GeneratorConfig::default()),

            toast_message: None,
        }
    }

    fn show_toast(&mut self, msg: impl Into<String>) {
        self.toast_message = Some((msg.into(), std::time::Instant::now()));
    }

    fn lock_vault(&mut self) {
        self.is_locked = true;
        self.vault = None;
        self.key = None;
        self.master_password_input.clear();
        self.confirm_password_input.clear();
        self.show_entry_dialog = false;
        self.show_toast("Vault locked securely");
    }
}

fn dirs_vault_path() -> PathBuf {
    if let Some(proj_dirs) = directories::ProjectDirs::from("com", "pranav256", "rusty-vault") {
        let dir = proj_dirs.data_local_dir();
        dir.join("vault.enc")
    } else {
        PathBuf::from("vault.enc")
    }
}

impl eframe::App for RustyVaultApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_header").show(ctx, |ui| {
            ui.add_space(6.0);
            ui.horizontal(|ui| {
                ui.heading(RichText::new("🔐 RustyVault").strong());
                ui.label(RichText::new("v0.1.0").weak());
            });
            ui.add_space(6.0);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.is_locked {
                self.render_locked_view(ui);
            } else {
                ui.label("Vault unlocked!");
            }
        });
    }
}

impl RustyVaultApp {
    fn render_locked_view(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(50.0);
            ui.label(RichText::new("Enter Master Password").size(22.0).strong());
            ui.add_space(8.0);

            if self.is_new_vault {
                ui.label(RichText::new("No existing vault found. Create a strong master password to initialize your encrypted vault.").color(Color32::LIGHT_GRAY));
            } else {
                ui.label(RichText::new("Vault is encrypted with AES-256-GCM & Argon2id.").weak());
            }

            ui.add_space(20.0);

            ui.horizontal(|ui| {
                ui.add_space(ui.available_width() / 2.0 - 150.0);
                ui.vertical(|ui| {
                    ui.label("Master Password:");
                    let pass_edit = ui.add(
                        egui::TextEdit::singleline(&mut self.master_password_input)
                            .password(true)
                            .desired_width(300.0),
                    );

                    if self.is_new_vault {
                        ui.add_space(8.0);
                        ui.label("Confirm Master Password:");
                        ui.add(
                            egui::TextEdit::singleline(&mut self.confirm_password_input)
                                .password(true)
                                .desired_width(300.0),
                        );
                    }

                    ui.add_space(16.0);

                    let action_label = if self.is_new_vault { "Create Vault" } else { "Unlock Vault" };
                    if ui.button(RichText::new(action_label).size(16.0)).clicked() {
                        // Will connect unlock handler in next step
                    }
                });
            });
        });
    }
}
