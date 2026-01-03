use ironbar_script_net::diff::NetSpeed;
use ironbar_script_net::poll::poll;
use ironbar_script_net::read::NetBytes;
use tokio_stream::StreamExt;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let s = poll(
        || async { NetBytes::read_total_net_bytes().await.unwrap() },
        1000,
    );
    let mut s = Box::pin(s);
    let mut prev: Option<NetBytes> = None;

    while let Some(curr) = s.next().await {
        if let Some(prev_) = prev {
            let speed = NetSpeed::diff_speed(prev_, curr);
            print(speed);
        }
        prev = Some(curr);
    }

    Ok(())
}

fn print(speed: NetSpeed) {
    println!(
        "{}",
        ironbar_script_net::html! {
            span {
                small {
                    {speed.tx}
                    " "
                    sup { span(color="#A3BE8C") { "" } }
                    "  "
                    {speed.rx}
                    " "
                    sup { span(color="#BF616A") { "" } }
                }
            }
        }
    );
}
