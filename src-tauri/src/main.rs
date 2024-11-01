// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn fetch_projects() -> Vec<Project> {
    // Implement your logic to fetch projects from a database or file
    vec![
        Project {
            id: 1,
            name: "New Residential Complex".to_string(),
            status: "In Progress".to_string(),
        },
        Project {
            id: 2,
            name: "City Center Renovation".to_string(),
            status: "Planning".to_string(),
        },
        Project {
            id: 3,
            name: "Wallis Administration Building".to_string(),
            status: "In Construction".to_string(),
        }
    ]
}

#[tauri::command]
fn fetch_tasks() -> Vec<Task> {
    // Implémentez votre logique pour récupérer les tâches
    vec![
        Task {
            id: 1,
            title: "Finaliser les plans".to_string(),
            assignee: "Alice".to_string(),
            due_date: "2023-07-15".to_string(),
            completed: false,
        },
        Task {
            id: 2,
            title: "Obtenir les permis".to_string(),
            assignee: "Bob".to_string(),
            due_date: "2023-07-30".to_string(),
            completed: true,
        }
        // ... autres tâches
    ]
}

#[tauri::command]
fn fetch_team() -> Vec<TeamMember> {
    // Implémentez votre logique pour récupérer les membres de l'équipe
    vec![
        TeamMember {
            id: 1,
            name: "Alice Dupont".to_string(),
            role: "Architecte en chef".to_string(),
            avatar: "https://i.pravatar.cc/250?img=32".to_string(),
        },
        TeamMember { 
            id: 2, 
            name: "Bob Martin".to_string(), 
            role: "Chef de projet".to_string(), 
            avatar: "https://i.pravatar.cc/250?img=12".to_string(),
        }
        // ... autres membres de l'équipe
    ]
}

#[derive(serde::Serialize)]
struct Project {
    id: u32,
    name: String,
    status: String,
}

#[derive(serde::Serialize)]
struct Task {
    id: u32,
    title: String,
    assignee: String,
    due_date: String,
    completed: bool,
}

#[derive(serde::Serialize)]
struct TeamMember {
    id: u32,
    name: String,
    role: String,
    avatar: String,
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![fetch_projects, fetch_tasks, fetch_team])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
