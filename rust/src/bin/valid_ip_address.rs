// https://leetcode.com/problems/validate-ip-address/

impl Solution {
    pub fn valid_ip_address(query_ip: String) -> String {
        let mut result: String = "Neither".to_string();
        let ipv4_split: Vec<String> = query_ip.split('.').map(|s| s.to_string()).collect();
        let ipv6_split: Vec<String> = query_ip.split(':').map(|s| s.to_string()).collect();

        if !(ipv4_split.len() == 4 || ipv6_split.len() == 8) {
            return result;
        } else if ipv4_split.len() == 4 {
            for ipv4 in ipv4_split {
                if ipv4.len() == 0
                    || ipv4.len() > 3
                    || (ipv4.len() > 1 && ipv4.chars().nth(0).unwrap() == '0')
                {
                    return result;
                }
                if let Ok(ip) = u64::from_str_radix(&ipv4, 10) {
                    if ip > 255 {
                        return result;
                    }
                } else {
                    return result;
                }
            }
            result = "IPv4".to_string();
        } else if ipv6_split.len() == 8 {
            for ipv6 in ipv6_split {
                if ipv6.len() == 0 || ipv6.len() > 4 {
                    return result;
                }
                if let Ok(_) = u64::from_str_radix(&ipv6, 16) {
                    continue;
                } else {
                    return result;
                }
            }
            result = "IPv6".to_string();
        }
        return result;
    }
}

pub struct Solution {}

fn main() {
    println!("{}", Solution::valid_ip_address("172.16.254.1".to_string()));
    println!("{}", Solution::valid_ip_address("1.0.1.".to_string()));
    println!(
        "{}",
        Solution::valid_ip_address("2001:0db8:85a3:0:0:8A2E:0370:7334".to_string())
    );
    println!(
        "{}",
        Solution::valid_ip_address("256.256.256.256".to_string())
    );
    println!(
        "{}",
        Solution::valid_ip_address("2001:0db8:85a3::8A2E:037j:7334".to_string())
    );
    println!(
        "{}",
        Solution::valid_ip_address("02001:0db8:85a3:0000:0000:8a2e:0370:7334".to_string())
    );
}
