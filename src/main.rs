use std::process::exit;
use steamworks::Client;

fn main() {
    let mut args = std::env::args();
    let _binary_name = args.next().unwrap();
    let usage_str = format!("Usage {} <AppId> <achievement name>", _binary_name);
    let app_id = args.next().expect(usage_str.as_str());

    let achievement_name = args.next().expect(usage_str.as_str());

    std::fs::write("steam_appid.txt", app_id).expect("Failed to write appid file");

    let (client, single) = Client::init().unwrap();

    let user_stats = client.user_stats();
    user_stats.request_current_stats();

    let (tx_user_stats_received, rx_user_stats_received) = std::sync::mpsc::channel();
    let _cb_user_stats = client.register_callback(move |val: steamworks::UserStatsReceived| {
        tx_user_stats_received.send(val).expect("Failed to send");
    });

    let (tx_user_achievement_stored, rx_user_achievement_stored) = std::sync::mpsc::channel();
    let _cb_user_achievement_stored =
        client.register_callback(move |val: steamworks::UserAchievementStored| {
            tx_user_achievement_stored
                .send(val)
                .expect("Failed to send UserAchievementStored");
        });

    let (tx_user_stats_stored, rx_user_stats_stored) = std::sync::mpsc::channel();
    let _cb_user_stats_stored =
        client.register_callback(move |val: steamworks::UserStatsStored| {
            tx_user_stats_stored
                .send(val)
                .expect("Failed to send UserAchievementStored");
        });

    loop {
        single.run_callbacks();

        if let Ok(res) = rx_user_stats_received.try_recv() {
            println!("Got callback: {:?}", res);
            if res.result.is_err() {
                panic!("Error with {:?}", res);
            }
            println!("Stats received for game: {:?}", res);

            let achievement = user_stats.achievement(achievement_name.as_str());

            let ach_val = achievement.get().expect("Failed to get achievement");
            println!("Unlocked achievement? {}", ach_val);

            achievement.set().expect("Failed to unlock");

            println!("Stored achievements");

            let ach_val = achievement.get().expect("Failed to get achievement");
            println!("Unlocked achievement? {}", ach_val);

            if let Ok(_res) = user_stats.store_stats() {
                println!("Stored stats");
            } else {
                panic!("Failed to store stats");
            }
        }

        if let Ok(res) = rx_user_achievement_stored.try_recv() {
            println!("Got callback: {:?}", res);
        }

        if let Ok(res) = rx_user_stats_stored.try_recv() {
            println!("User stats stored: {:?}", res);
            exit(0);
        }

        ::std::thread::sleep(::std::time::Duration::from_millis(100));
    }
}
