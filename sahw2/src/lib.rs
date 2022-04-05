//!
//! Library for Substrate basic homework2
//!

#![deny(warnings)]
#![deny(missing_docs)]

/// parse cli parameters
pub mod mcli {
    use std::str::FromStr;
    use std::{
        env,
        net::{IpAddr, Ipv4Addr},
    };

    #[derive(Debug)]
    /// Parameters specified by Cli
    pub struct CliParams {
        /// Listened Ip address
        pub ia: IpAddr,
        /// Bind port
        pub port: u16,
    }

    impl Default for CliParams {
        fn default() -> Self {
            CliParams {
                ia: IpAddr::from(Ipv4Addr::new(127, 0, 0, 1)),
                port: 0,
            }
        }
    }

    /// A simple parser to extract tcpserver parameters
    pub fn parse_args() -> Option<CliParams> {
        let args = env::args().collect::<Vec<_>>();
        let len = args.len();
        if len == 1 {
            Some(Default::default())
        } else if len == 2 {
            if let Ok(ia) = IpAddr::from_str(&args[1]) {
                Some(CliParams {
                    ia,
                    ..Default::default()
                })
            } else {
                None
            }
        } else if len >= 3 {
            if let Ok(ia) = IpAddr::from_str(&args[1]) {
                if let Ok(port) = args[2].parse::<u16>() {
                    Some(CliParams { ia, port })
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}
