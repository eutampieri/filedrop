pub type Pool = Vec<(String, u64, u64)>;

fn get_unsorted_pool() -> Result<Pool, std::io::Error> {
    Ok(std::fs::read_dir("pool/")?
        .filter_map(|x| {
            x.ok().map(|y| {
                (
                    "pool/".to_owned() + &y.file_name().into_string().unwrap_or_default(),
                    y.metadata().ok().map(|z| z.len()).unwrap_or_default(),
                    y.metadata()
                        .ok()
                        .map(|z| {
                            z.created()
                                .map(|a| {
                                    a.duration_since(std::time::UNIX_EPOCH)
                                        .unwrap_or_default()
                                        .as_secs()
                                })
                                .unwrap_or_default()
                        })
                        .unwrap_or_default(),
                )
            })
        })
        .collect())
}

pub fn get_pool() -> Result<Pool, std::io::Error> {
    let mut pool = get_unsorted_pool()?;
    sort_pool(&mut pool);
    Ok(pool)
}

fn sort_pool(pool: &mut Pool) {
    pool.sort_by(|a, b| (b.2).cmp(&a.2))
}

pub fn get_pool_size(pool: &Pool) -> u64 {
    pool.iter().map(|x| x.1).sum()
}
