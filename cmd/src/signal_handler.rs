// Copyright 2017 TiKV Project Authors. Licensed under Apache-2.0.

pub use self::imp::wait_for_signal;

#[cfg(unix)]
mod imp {
    use engine_rocks::RocksEngine;
    use engine_traits::{Engines, RaftEngine};

    #[allow(dead_code)]
    pub fn wait_for_signal<ER: RaftEngine>(_: Option<Engines<RocksEngine, ER>>) {
        loop {}
        // let trap = Trap::trap(&[SIGTERM]);
        // for sig in trap {
        //     match sig {
        //         SIGTERM => {
        //             info!("receive signal {}, stopping server...", sig as c_int);
        //             break;
        //         }
        //         SIGUSR1 => {
        //             // Use SIGUSR1 to log metrics.
        //             info!("{}", metrics::dump());
        //             if let Some(ref engines) = engines {
        //                 info!("{:?}", MiscExt::dump_stats(&engines.kv));
        //                 info!("{:?}", RaftEngine::dump_stats(&engines.raft));
        //             }
        //         }
        //         // TODO: handle more signal
        //         _ => unreachable!(),
        //     }
        // }
    }
}

#[cfg(not(unix))]
mod imp {
    use engine_rocks::RocksEngine;
    use engine_traits::Engines;

    pub fn wait_for_signal(_: Option<Engines<RocksEngine, RocksEngine>>) {}
}
