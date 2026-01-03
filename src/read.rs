use tokio::fs;

const SYS_NET_PATH: &str = "/sys/class/net";
const RX_STAT_PATH: &str = "/statistics/rx_bytes";
const TX_STAT_PATH: &str = "/statistics/tx_bytes";

#[derive(Debug, Clone, Copy)]
pub struct NetBytes {
    pub rx: u64, // received bytes
    pub tx: u64, // transmitted bytes
    pub time: u128,
}

impl NetBytes {
    async fn get_ifaces() -> anyhow::Result<Vec<String>> {
        let mut entries = fs::read_dir(SYS_NET_PATH).await?;
        let mut ifaces = Vec::new();

        while let Some(entry) = entries.next_entry().await? {
            let name = entry.file_name();
            if name != "lo" {
                ifaces.push(name.to_string_lossy().into_owned());
            }
        }

        Ok(ifaces)
    }

    async fn read_bytes(iface: String) -> anyhow::Result<(u64, u64)> {
        let (rx, tx) = tokio::try_join!(
            fs::read_to_string(format!("{SYS_NET_PATH}/{iface}/{RX_STAT_PATH}")),
            fs::read_to_string(format!("{SYS_NET_PATH}/{iface}/{TX_STAT_PATH}")),
        )?;
        let (rx, tx) = (rx.trim().parse()?, tx.trim().parse()?);
        Ok((rx, tx))
    }

    pub async fn read_total_net_bytes() -> anyhow::Result<Self> {
        let mut sum = (0u64, 0u64);

        for iface in Self::get_ifaces().await? {
            let (rx, tx) = Self::read_bytes(iface).await?;
            sum.0 += rx;
            sum.1 += tx;
        }

        Ok(NetBytes {
            rx: sum.0,
            tx: sum.1,
            time: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_millis(),
        })
    }
}
