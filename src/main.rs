use std::sync::Arc;
use tokio::{fs, sync::Semaphore};
use rayon::prelude::*;
use libc;

fn get_max_open_files() -> usize {
    let mut lim = libc::rlimit {
        rlim_cur: 0,
        rlim_max: 0,
    };
    let ret = unsafe { libc::getrlimit(libc::RLIMIT_NOFILE, &mut lim) };
    if ret == 0 {
        lim.rlim_cur as usize
    } else {
        1024
    }
}

async fn search_directory(
    dir_path: String,
    con_text: Arc<String>,
    sem: Arc<Semaphore>,
) -> std::io::Result<()> {
    let mut entries = fs::read_dir(dir_path).await?;
    let mut file_paths = Vec::new();
    let mut join_handles = Vec::new();

    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        if path.is_dir() {
            let dir_clone = path.clone();
            let content_clone = Arc::clone(&con_text);
            // İki nüsxə yaradırıq: biri icazə üçün, biri closure üçün
            let sem_clone = sem.clone();
            let sem_for_permit = sem.clone();
            // İcazə almaq üçün semaforu istifadə edirik
            let permit = sem_for_permit.acquire_owned().await.unwrap();

            let handle = tokio::task::spawn_local(async move {
                let result = search_directory(
                    dir_clone.to_str().unwrap().to_string(),
                    content_clone,
                    sem_clone,
                )
                .await;
                // Tapşırıq bitdikdə permit avtomatik drop ediləcək, beləliklə resurs azad olunur.
                drop(permit);
                result.unwrap();
            });
            join_handles.push(handle);
        } else if path.is_file() {
            file_paths.push(path);
        }
      
    }

    file_paths.into_par_iter().for_each(|file_path| {
        if let Ok(content) = std::fs::read_to_string(&file_path) {
            if content.contains(&con_text[..]) {
                println!("Tapıldı: {:?}", file_path);
            }
        }
    });

    for handle in join_handles {
        handle.await.unwrap();
    }

    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let local = tokio::task::LocalSet::new();
    let max_files = get_max_open_files();
    println!("Sistem maksimum fayl descriptor sayı: {}", max_files);
    let sem = Arc::new(Semaphore::new(max_files));

    local.run_until(async {
        println!("Salam Aleykum, Pls enter directory path:");
        let mut dir_path = String::new();
        std::io::stdin()
            .read_line(&mut dir_path)
            .expect("Error: Cannot read directory path!");

        println!("Pls enter context text:");
        let mut con_text = String::new();
        std::io::stdin()
            .read_line(&mut con_text)
            .expect("Error: Cannot read context text!");

        let con_text = Arc::new(con_text.trim().to_string());
        let dir_path = dir_path.trim().to_string();

        let _ = search_directory(dir_path, con_text, sem).await;
    })
    .await;
}
