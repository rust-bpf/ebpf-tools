use bcc::core::BPF;
use clap::{App, Arg};
use failure::Error;

use core::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{thread, time};

// A simple tool for reporting runqueue latency
//
// Based on: https://github.com/iovisor/bcc/blob/master/tools/runqlat.py

fn attach_events(bpf: &mut BPF) {
    if bpf.support_raw_tracepoint() {
        let raw_tp_sched_wakeup = bpf.load_raw_tracepoint("raw_tp__sched_wakeup").unwrap();
        let raw_tp_sched_wakeup_new = bpf.load_raw_tracepoint("raw_tp__sched_wakeup_new").unwrap();
        let raw_tp_sched_switch = bpf.load_raw_tracepoint("raw_tp__sched_switch").unwrap();

        bpf.attach_raw_tracepoint("sched_wakeup", raw_tp_sched_wakeup)
            .unwrap();
        bpf.attach_raw_tracepoint("sched_wakeup_new", raw_tp_sched_wakeup_new)
            .unwrap();
        bpf.attach_raw_tracepoint("sched_switch", raw_tp_sched_switch)
            .unwrap();
    } else {
        // load + attach kprobes!
        let trace_run = bpf.load_kprobe("trace_run").unwrap();
        let trace_ttwu_do_wakeup = bpf.load_kprobe("trace_ttwu_do_wakeup").unwrap();
        let trace_wake_up_new_task = bpf.load_kprobe("trace_wake_up_new_task").unwrap();

        bpf.attach_kprobe("finish_task_switch", trace_run).unwrap();
        bpf.attach_kprobe("wake_up_new_task", trace_wake_up_new_task)
            .unwrap();
        bpf.attach_kprobe("ttwu_do_wakeup", trace_ttwu_do_wakeup)
            .unwrap();
    }
}

fn do_main(runnable: Arc<AtomicBool>) -> Result<(), Error> {
    let matches = App::new("runqlat")
        .about("Reports distribution of scheduler latency")
        .arg(
            Arg::with_name("interval")
                .long("interval")
                .value_name("Seconds")
                .help("Integration window duration and period for stats output")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("windows")
                .long("windows")
                .value_name("Count")
                .help("The number of intervals before exit")
                .takes_value(true),
        )
        .get_matches();

    let interval: usize = matches
        .value_of("interval")
        .unwrap_or("1")
        .parse()
        .expect("Invalid argument for interval");
    let windows: Option<usize> = matches
        .value_of("windows")
        .map(|v| v.parse().expect("Invalid argument for windows"));

    let code = if cfg!(any(feature = "bcc_v0_4_0", feature = "bcc_v0_5_0")) {
        include_str!("bpf.c").replace("#define FEATURE_SUPPORT_RAW_TP", "")
    } else {
        include_str!("bpf.c").to_string()
    };
    // compile the above BPF code!
    let mut bpf = BPF::new(&code)?;
    attach_events(&mut bpf);

    let table = bpf.table("dist");
    let mut window = 0;

    while runnable.load(Ordering::SeqCst) {
        thread::sleep(time::Duration::new(interval as u64, 0));
        println!("======");
        let mut overflow = 0;
        for (power, entry) in table.iter().enumerate() {
            let value = entry.value;

            let mut v = [0_u8; 8];
            for (i, byte) in v.iter_mut().enumerate() {
                *byte = *value.get(i).unwrap_or(&0);
            }
            let count = u64::from_ne_bytes(v);
            let value = 2_u64.pow(power as u32);
            if value < 1_000_000 {
                println!("{} uS: {}", 2_u64.pow(power as u32), count);
            } else {
                overflow += count;
            }
        }
        println!("> 1 S: {}", overflow);
        if let Some(windows) = windows {
            window += 1;
            if window >= windows {
                return Ok(());
            }
        }
    }
    Ok(())
}

fn main() {
    let runnable = Arc::new(AtomicBool::new(true));
    let r = runnable.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Failed to set handler for SIGINT / SIGTERM");

    if let Err(x) = do_main(runnable) {
        eprintln!("Error: {}", x);
        eprintln!("{}", x.backtrace());
        std::process::exit(1);
    }
}
