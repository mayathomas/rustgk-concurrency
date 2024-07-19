use std::{thread, time::Duration};

use anyhow::Result;
use rand::Rng;
use rustgk_concurrency::AmapMetrics;

const N: usize = 2;
const M: usize = 4;

fn main() -> Result<()> {
    let metrics = AmapMetrics::new(&[
        "call.thread.worker.0",
        "call.thread.worker.1",
        "req.page.1",
        "req.page.2",
        "req.page.3",
        "req.page.4",
    ]);

    println!("{}", metrics);

    for idx in 0..N {
        task_worker(idx, metrics.clone())?;
    }

    for _ in 0..M {
        request_worker(metrics.clone())?;
    }

    loop {
        thread::sleep(Duration::from_secs(2));
        println!("{}", metrics);
    }
}

fn task_worker(idx: usize, metrics: AmapMetrics) -> Result<()> {
    thread::spawn(move || {
        loop {
            //do something...
            let mut rng = rand::thread_rng();
            thread::sleep(Duration::from_millis(rng.gen_range(100..5000)));
            metrics.inc(format!("call.thread.worker.{}", idx))?;
        }
        //和metrics.inc(format!("req.page.{}", page))?;这一行的返回值要保持一致，都要是Result<(), anyhow::Error>
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });
    Ok(())
}

fn request_worker(metrics: AmapMetrics) -> Result<()> {
    thread::spawn(move || {
        loop {
            //do something...
            let mut rng = rand::thread_rng();
            thread::sleep(Duration::from_millis(rng.gen_range(50..800)));
            let page = rng.gen_range(1..5);
            metrics.inc(format!("req.page.{}", page))?;
        }
        //和metrics.inc(format!("req.page.{}", page))?;这一行的返回值要保持一致，都要是Result<(), anyhow::Error>
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });
    Ok(())
}
