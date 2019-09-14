use futures_util::stream::StreamExt;
use std::time::Duration;

use tiny::ServerInfo;

fn main() {
    let executor = tokio::runtime::Runtime::new().unwrap();

    let server_info = ServerInfo {
        addr: "chat.freenode.net".to_string(),
        port: 6667,
        pass: None,
        hostname: "omer".to_string(),
        realname: "omer".to_string(),
        nicks: vec!["osa1".to_string()],
        auto_join: vec![],
        nickserv_ident: None,
    };

    executor.spawn(async {
        let (_client, mut rcv_ev) = tiny::Client::new(server_info);
        println!("client created");
        while let Some(ev) = rcv_ev.next().await {
            println!("ev: {:?}", ev);
        }
    });

    let server_info = ServerInfo {
        addr: "chat.freenode.net".to_string(),
        port: 6667,
        pass: None,
        hostname: "omer".to_string(),
        realname: "omer".to_string(),
        nicks: vec!["osa1s_irc_bot".to_string()],
        auto_join: vec!["#justtesting".to_string()],
        nickserv_ident: None,
    };

    executor.spawn(async {
        println!("Sleeping for 3 seconds before the second connection");
        tokio::timer::delay(tokio::clock::now() + Duration::from_secs(3)).await;
        let (mut client, mut rcv_ev) = tiny::Client::new(server_info);

        println!("client created, spawning incoming msg handler task");

        tokio::spawn(async move {
            while let Some(ev) = rcv_ev.next().await {
                println!("ev: {:?}", ev);
            }
        });

        println!("sleeping for 5 seconds before joining #justtesting");
        tokio::timer::delay(tokio::clock::now() + Duration::from_secs(5)).await;
        client.join("#justtesting");
    });

    executor.shutdown_on_idle();
}