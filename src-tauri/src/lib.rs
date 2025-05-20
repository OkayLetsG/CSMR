use tauri::Manager;
use tauri_plugin_sql::{Builder, Migration, MigrationKind};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let _migrations = vec![
        Migration {
            version: 1,
            description: "create_folders_table",
            sql: "CREATE TABLE IF NOT EXISTS FOLDERS (
									FID INTEGER PRIMARY KEY AUTOINCREMENT,
									FNAME TEXT NOT NULL,
									FPARENT_ID INTEGER,
									FCREATED_AT TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
									FMODIFIED_AT TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                                    FISDELETED INTEGER DEFAULT 0,
									FOREIGN KEY (FPARENT_ID) REFERENCES FOLDERS(FID) ON DELETE CASCADE
                                    FOREIGN KEY (FPARENT_ID) REFERENCES FOLDERS(FID) ON UPDATE CASCADE
                )",
            kind: MigrationKind::Up,
        },
        Migration {
            version: 2,
            description: "create_snippets_table",
            sql: "CREATE TABLE IF NOT EXISTS SNIPPETS (
									SID INTEGER PRIMARY KEY AUTOINCREMENT,
									STITLE TEXT NOT NULL,
									SCREATED_AT TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
									SMODIFIED_AT TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
									SFID INTEGER,
                                    SLNAME TEXT,
                                    SLDOCS TEXT,
                                    SISDELETED INTEGER DEFAULT 0,
									FOREIGN KEY (SFID) REFERENCES FOLDERS(FID) ON DELETE CASCADE
				)",
            kind: MigrationKind::Up,
        },
        Migration {
            version: 3,
            description: "create_fragments_table",
            sql: "CREATE TABLE IF NOT EXISTS FRAGMENTS (
									FRID INTEGER PRIMARY KEY AUTOINCREMENT,
									FRSID INTEGER NOT NULL,
									FRNAME TEXT NOT NULL,
									FRCONTENT TEXT,
									FRCREATED_AT TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
									FRMODIFIED_AT TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                                    FRISDELETED INTEGER DEFAULT 0,
									FOREIGN KEY (FRSID) REFERENCES SNIPPETS(SID) ON DELETE CASCADE
								)",
            kind: MigrationKind::Up,
        }
    ];
    tauri::Builder::default()
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations("sqlite:csmremasterd_database.db", _migrations)
                .build(),
        )
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
