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

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if !self.is_locked {
                        if ui.button("🔒 Lock Vault").clicked() {
                            self.lock_vault();
                        }
                    } else {
                        ui.label(RichText::new("Locked").weak());
                    }
                });
            });
            ui.add_space(6.0);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.is_locked {
                self.render_locked_view(ui);
            } else {
                self.render_unlocked_view(ui);
            }
        });

        if self.show_entry_dialog {
            self.render_entry_dialog(ctx);
        }
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

                    if let Some(err) = &self.unlock_error {
                        ui.add_space(8.0);
                        ui.label(RichText::new(err).color(Color32::from_rgb(240, 80, 80)));
                    }

                    ui.add_space(16.0);

                    let action_label = if self.is_new_vault { "Create Vault" } else { "Unlock Vault" };
                    let enter_pressed = pass_edit.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));

                    if (ui.button(RichText::new(action_label).size(16.0)).clicked() || enter_pressed)
                        && !self.master_password_input.is_empty()
                    {
                        self.handle_unlock_or_create();
                    }

                    ui.add_space(24.0);
                    ui.separator();
                    ui.add_space(12.0);

                    if ui.button("📥 Import from legacy Python emails.txt").clicked() {
                        self.handle_legacy_import();
                    }
                });
            });
        });
    }

    fn handle_unlock_or_create(&mut self) {
        if self.is_new_vault {
            if self.master_password_input != self.confirm_password_input {
                self.unlock_error = Some("Passwords do not match".to_string());
                return;
            }
            if self.master_password_input.len() < 8 {
                self.unlock_error =
                    Some("Master password should be at least 8 characters".to_string());
                return;
            }

            let vault = Vault::new();
            match crypto::derive_key(&self.master_password_input, &vault.salt) {
                Ok(key) => {
                    if let Err(e) = vault.save_to_file(&self.vault_path, &key) {
                        self.unlock_error = Some(format!("Failed to save vault: {}", e));
                        return;
                    }
                    self.unlock_error = None;
                    self.is_new_vault = false;
                    self.vault = Some(vault);
                    self.key = Some(key);
                    self.is_locked = false;
                    self.show_toast("New vault created successfully!");
                }
                Err(e) => {
                    self.unlock_error = Some(format!("Key derivation error: {}", e));
                }
            }
        } else {
            match Vault::load_from_file(&self.vault_path, &self.master_password_input) {
                Ok((vault, key)) => {
                    self.unlock_error = None;
                    let count = vault.entries.len();
                    self.vault = Some(vault);
                    self.key = Some(key);
                    self.is_locked = false;
                    self.show_toast(format!("Vault unlocked! ({} entries)", count));
                }
                Err(_) => {
                    self.unlock_error =
                        Some("Incorrect master password or corrupted vault file".to_string());
                }
            }
        }
    }

    fn handle_legacy_import(&mut self) {
        let legacy_file = Path::new("emails.txt");
        if !legacy_file.exists() {
            self.unlock_error =
                Some("Legacy 'emails.txt' not found in current directory".to_string());
            return;
        }

        if self.master_password_input.is_empty() {
            self.unlock_error =
                Some("Enter master password above first to encrypt imported items".to_string());
            return;
        }

        let mut vault = Vault::new();
        match vault.import_legacy_txt(legacy_file) {
            Ok(count) => match crypto::derive_key(&self.master_password_input, &vault.salt) {
                Ok(key) => {
                    let _ = vault.save_to_file(&self.vault_path, &key);
                    self.is_new_vault = false;
                    self.vault = Some(vault);
                    self.key = Some(key);
                    self.is_locked = false;
                    self.show_toast(format!(
                        "Successfully imported {} accounts from legacy Python format!",
                        count
                    ));
                }
                Err(e) => self.unlock_error = Some(e.to_string()),
            },
            Err(e) => self.unlock_error = Some(format!("Import failed: {}", e)),
        }
    }

    fn render_unlocked_view(&mut self, ui: &mut egui::Ui) {
        let total_count = self.vault.as_ref().map(|v| v.entries.len()).unwrap_or(0);

        ui.horizontal(|ui| {
            ui.add(
                egui::TextEdit::singleline(&mut self.search_query)
                    .hint_text("🔍 Search accounts, emails, services...")
                    .desired_width(280.0),
            );

            if ui.button("➕ Add Entry").clicked() {
                self.open_add_dialog();
            }

            if ui.button("🎲 Generate Password").clicked() {
                self.show_generator_modal = true;
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(RichText::new(format!("Total: {} accounts", total_count)).weak());
            });
        });

        ui.add_space(10.0);
        ui.separator();
        ui.add_space(6.0);

        let filtered_entries: Vec<VaultEntry> = self
            .vault
            .as_ref()
            .map(|v| v.search(&self.search_query).into_iter().cloned().collect())
            .unwrap_or_default();

        let mut entry_to_delete: Option<String> = None;
        let mut entry_to_edit: Option<VaultEntry> = None;
        let mut toast_to_show: Option<String> = None;

        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Grid::new("vault_entries_grid")
                .striped(true)
                .min_col_width(120.0)
                .spacing([14.0, 10.0])
                .show(ui, |ui| {
                    ui.label(RichText::new("Service / Title").strong());
                    ui.label(RichText::new("Username / Email").strong());
                    ui.label(RichText::new("Password").strong());
                    ui.label(RichText::new("Actions").strong());
                    ui.end_row();

                    for entry in &filtered_entries {
                        ui.label(RichText::new(&entry.service).strong());

                        let user_email = if !entry.username.is_empty() && !entry.email.is_empty() {
                            format!("{} ({})", entry.username, entry.email)
                        } else if !entry.username.is_empty() {
                            entry.username.clone()
                        } else {
                            entry.email.clone()
                        };
                        ui.label(user_email);

                        ui.label("••••••••••••");

                        ui.horizontal(|ui| {
                            if ui.button("📋 Copy Pass").clicked() {
                                ui.output_mut(|o| o.copied_text = entry.password.clone());
                                toast_to_show =
                                    Some(format!("Copied password for {}", entry.service));
                            }

                            if ui.button("📋 Copy User").clicked() {
                                let to_copy = if !entry.username.is_empty() {
                                    &entry.username
                                } else {
                                    &entry.email
                                };
                                ui.output_mut(|o| o.copied_text = to_copy.clone());
                                toast_to_show = Some("Copied username to clipboard".to_string());
                            }

                            if ui.button("✏ Edit").clicked() {
                                entry_to_edit = Some(entry.clone());
                            }

                            if ui
                                .button(RichText::new("🗑").color(Color32::from_rgb(220, 80, 80)))
                                .clicked()
                            {
                                entry_to_delete = Some(entry.id.clone());
                            }
                        });

                        ui.end_row();
                    }
                });
        });

        if let Some(msg) = toast_to_show {
            self.show_toast(msg);
        }

        if let Some(id) = entry_to_delete {
            if let Some(vault) = &mut self.vault {
                vault.remove_entry(&id);
                if let Some(key) = &self.key {
                    let _ = vault.save_to_file(&self.vault_path, key);
                }
            }
            self.show_toast("Entry deleted");
        }

        if let Some(entry) = entry_to_edit {
            self.open_edit_dialog(entry);
        }
    }

    fn open_add_dialog(&mut self) {
        self.editing_entry = None;
        self.edit_service.clear();
        self.edit_username.clear();
        self.edit_email.clear();
        self.edit_password.clear();
        self.edit_url.clear();
        self.edit_notes.clear();
        self.edit_show_password = false;
        self.show_entry_dialog = true;
    }

    fn open_edit_dialog(&mut self, entry: VaultEntry) {
        self.edit_service = entry.service.clone();
        self.edit_username = entry.username.clone();
        self.edit_email = entry.email.clone();
        self.edit_password = entry.password.clone();
        self.edit_url = entry.url.clone();
        self.edit_notes = entry.notes.clone();
        self.edit_show_password = false;
        self.editing_entry = Some(entry);
        self.show_entry_dialog = true;
    }

    fn render_entry_dialog(&mut self, ctx: &egui::Context) {
        let is_editing = self.editing_entry.is_some();
        let title = if is_editing {
            "Edit Account"
        } else {
            "Add New Account"
        };

        let mut close_dialog = false;
        let mut save_entry = false;

        egui::Window::new(title)
            .collapsible(false)
            .resizable(false)
            .default_size([450.0, 380.0])
            .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.label("Service / Website Name *");
                    ui.add(egui::TextEdit::singleline(&mut self.edit_service).desired_width(400.0));

                    ui.add_space(4.0);
                    ui.label("Username");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.edit_username).desired_width(400.0),
                    );

                    ui.add_space(4.0);
                    ui.label("Email");
                    ui.add(egui::TextEdit::singleline(&mut self.edit_email).desired_width(400.0));

                    ui.add_space(4.0);
                    ui.horizontal(|ui| {
                        ui.label("Password *");
                        if ui.button("🎲 Generate").clicked() {
                            self.edit_password = generate_password(&self.generator_config);
                        }
                    });

                    ui.horizontal(|ui| {
                        ui.add(
                            egui::TextEdit::singleline(&mut self.edit_password)
                                .password(!self.edit_show_password)
                                .desired_width(330.0),
                        );
                        if ui
                            .button(if self.edit_show_password {
                                "Hide"
                            } else {
                                "Show"
                            })
                            .clicked()
                        {
                            self.edit_show_password = !self.edit_show_password;
                        }
                    });

                    if !self.edit_password.is_empty() {
                        let entropy = calculate_entropy(&self.edit_password);
                        let (rating, (r, g, b)) = strength_rating(entropy);
                        ui.label(
                            RichText::new(format!("Strength: {} ({:.1} bits)", rating, entropy))
                                .color(Color32::from_rgb(r, g, b))
                                .size(11.0),
                        );
                    }

                    ui.add_space(4.0);
                    ui.label("Notes (Optional)");
                    ui.add(
                        egui::TextEdit::multiline(&mut self.edit_notes)
                            .desired_rows(3)
                            .desired_width(400.0),
                    );

                    ui.add_space(12.0);
                    ui.horizontal(|ui| {
                        if ui.button(RichText::new("Save Entry").strong()).clicked()
                            && !self.edit_service.trim().is_empty()
                        {
                            save_entry = true;
                        }
                        if ui.button("Cancel").clicked() {
                            close_dialog = true;
                        }
                    });
                });
            });

        if save_entry {
            let service_name = self.edit_service.clone();
            let is_update = self.editing_entry.is_some();

            if let Some(vault) = &mut self.vault {
                if let Some(existing) = &self.editing_entry {
                    let mut updated = existing.clone();
                    updated.service = self.edit_service.clone();
                    updated.username = self.edit_username.clone();
                    updated.email = self.edit_email.clone();
                    updated.password = self.edit_password.clone();
                    updated.url = self.edit_url.clone();
                    updated.notes = self.edit_notes.clone();
                    vault.update_entry(updated);
                } else {
                    let mut entry = VaultEntry::new(
                        &self.edit_service,
                        &self.edit_username,
                        &self.edit_email,
                        &self.edit_password,
                    );
                    entry.url = self.edit_url.clone();
                    entry.notes = self.edit_notes.clone();
                    vault.add_entry(entry);
                }
                if let Some(key) = &self.key {
                    let _ = vault.save_to_file(&self.vault_path, key);
                }
            }

            if is_update {
                self.show_toast(format!("Updated {}", service_name));
            } else {
                self.show_toast(format!("Saved {}", service_name));
            }
            close_dialog = true;
        }

        if close_dialog {
            self.show_entry_dialog = false;
        }
    }
}
