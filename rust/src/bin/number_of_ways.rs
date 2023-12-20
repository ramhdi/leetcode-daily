// https://leetcode.com/problems/number-of-ways-to-divide-a-long-corridor
// 2023/11/28

fn number_of_ways(corridor: String) -> i32 {
    const MOD: u64 = 1_000_000_007;
    let mut result: u64 = 1;
    let mut seat_count: u64 = 0;
    let mut plant_count: u64 = 0;

    for c in corridor.chars() {
        match c {
            'S' => {
                seat_count += 1;

                if seat_count % 2 == 1 {
                    if plant_count > 0 {
                        result = (result * (plant_count + 1)) % MOD;
                    }
                    plant_count = 0;
                }
            }
            'P' => {
                if seat_count > 0 && seat_count % 2 == 0 {
                    plant_count += 1;
                }
            }
            _ => (),
        }
    }

    if seat_count == 0 || seat_count % 2 == 1 {
        return 0;
    }
    return (result % MOD) as i32;
}

fn main() {
    println!("{}", number_of_ways("PPSPSPPPPSPSPPPPSPPSPPPPPSPPSPPPPPSPPPPSPPPPSPSPPPPSPPSPPPPSPPPPSPPPPPPSPPPSPPPPPPPSPPSPPPPPPSPSPPPPSPPPPSPPPPPPSPPPPSPPPPPPSPSPPPPPSSSSSSP".to_string()));
}
