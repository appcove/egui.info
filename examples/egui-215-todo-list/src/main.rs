use eframe::egui;
use std::collections::HashMap;

#[derive(Clone)]
struct TodoItem {
    name: String,
    body: String,
}

impl Default for TodoItem {
    fn default() -> Self {
        Self {
            name: "New Todo Item".to_owned(),
            body: String::new(),
        }
    }
}

struct ExampleApp {
    // The todo items, with a unique u32 identifier
    items: HashMap<u32, TodoItem>,
    // A counter to keep track of the next valid ID -- it is incremented every time it's used
    next_id: u32,

    // the id and item currently edited, if any
    currently_edited: Option<(u32, TodoItem)>,
}

impl ExampleApp {
    fn name() -> &'static str {
        "egui 215 todo list"
    }
}

impl Default for ExampleApp {
    fn default() -> Self {
        Self {
            items: HashMap::new(),
            next_id: 0,
            currently_edited: None,
        }
    }
}

impl eframe::App for ExampleApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Todo List");
            ui.monospace("Click 'Add Item' to add items to the list.");
            ui.monospace("Click on an item to view, edit, or remove it.");
            ui.monospace("The item will open in a new window.");
            ui.separator();

            // Create a clone of the todo list so that we can remove items while iterating
            // This also lets us use the id and items without a reference
            let items = self.items.clone();

            // Indent all todo items
            ui.indent("todo_items", |ui| {
                // For every item, show its name as a clickable label.
                for (id, item) in items {
                    // Add some spacing to let it breathe
                    ui.add_space(5.0);

                    // Add a clickable label using egui::Label::sense()
                    if ui
                        .add(egui::Label::new(&item.name).sense(egui::Sense::click()))
                        .clicked()
                    {
                        // Set this item to be the currently edited one
                        self.currently_edited = Some((id, item));
                    };

                    // Add some spacing to let it breathe
                    ui.add_space(5.0);
                }
            });

            // Add a button to add todo items
            if ui.button("Add Item").clicked() {
                // Add a default item at next_id
                self.items.insert(self.next_id, TodoItem::default());

                // And finally, increment next_id
                self.next_id += 1;
            }

            ui.separator();

            // Quit button, will close all windows
            if ui.button("Quit").clicked() {
                std::process::exit(0);
            }

            // If we're currently editing an item, we have to keep calling ctx.show_viewport_immediate
            // Remove the currently edited id and item, replace later if necessary
            if let Some((id, mut item)) = self.currently_edited.take() {
                let viewport_id = egui::ViewportId::from_hash_of(format!("edit {id}"));
                let viewport_builder = egui::ViewportBuilder::default()
                    .with_inner_size((300.0, 300.0))
                    .with_title(format!("edit {}", item.name));

                // This function is like eframe::App::update, except it can access ExampleApp as well
                let viewport_cb = |ctx: &egui::Context, _| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.label("Name:");
                        ui.text_edit_singleline(&mut item.name);
                        ui.label("Body:");
                        ui.text_edit_multiline(&mut item.body);

                        if ui.button("Save").clicked() {
                            // Insert our changed item at the id
                            self.items.insert(id, item);

                            // Set the currently edited item to nothing
                            self.currently_edited = None;
                        } else if ui.button("Cancel").clicked()
                            || ctx.input(|i| i.viewport().close_requested())
                        {
                            // Set the currently edited item to nothing
                            self.currently_edited = None;
                        } else if ui.button("Remove").clicked() {
                            // Remove the currently edited item
                            self.items.remove(&id);

                            // Set the currently edited item to nothing
                            self.currently_edited = None;
                        } else {
                            // Otherwise set the currently edited item to this item again so the window won't close
                            self.currently_edited = Some((id, item));
                        }
                    });
                };

                ctx.show_viewport_immediate(viewport_id, viewport_builder, viewport_cb);
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size((600.0, 600.0)),
        ..eframe::NativeOptions::default()
    };

    eframe::run_native(
        ExampleApp::name(),
        native_options,
        Box::new(|_| Box::<ExampleApp>::default()),
    )
}
