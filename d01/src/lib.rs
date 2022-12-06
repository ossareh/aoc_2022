use std::str; 

/// Parse list of calories elves are carrying
/// 
/// Each elf has been asked to write down the calories of each item of food they carry
/// Their lists are then concatenated together with an empty line delineating each set of entries
/// 
///     BOF:
///     100
///     200
/// 
///     300
///     400
/// 
///     600
///     EOF
/// 
/// Elf 1 has 300 calories, Elf 2 has 700 calories, Elf 3 has 600 calories
/// 
/// This function presupposes the file is correctly formatted.
pub fn compute(buf: Vec<u8>) -> Vec<u64>{

    // holder of elf records
    let mut records: Vec<u64> = vec![];
    
    // running count of calories current elf is holding
    let mut count = 0_u64;
    
    // indexes into the buffer
    // start is the byte offset of the current line
    // position is the byte we're currently reading
    let mut start = 0;
    let mut position = 0;

    fn push_elf_record(r: &mut Vec<u64>, c: &mut u64) {
        r.push(*c);
        *c = 0;
    }

    fn convert_calorie_count(b: &[u8], s: usize, p: usize) -> u64 {
        // this will panic loudly if the record is invalid
        let value = str::from_utf8(&b[s..p]).unwrap();
        value.parse::<u64>().unwrap()
    }

    (0..=buf.len()).for_each(|i| {
        if i == buf.len() {
            // handle final record
            count += convert_calorie_count(&buf, start, position);   
            push_elf_record(&mut records, &mut count);
        } else {

            match buf[i] {
                // new lines can either be an elf-record seperator or an calorie-record seperator
                // if position == start then it's an elf-record seperator
                0xA => {
                    if position == start {
                        // elf-record seperator
                        push_elf_record(&mut records, &mut count);
                    } else {
                        // calorie-record sperator
                        count += convert_calorie_count(&buf, start, position);   
                    }
                    // move start, and position idicies to the next byte
                    start = i + 1; 
                    position = i + 1; 
                    

                }
                _ => {
                    // keep reading
                    position = i + 1;
                }
            }
        }
    });

    
    records

}

#[cfg(test)]
pub mod test {
    use super::*;

    fn test_data() -> &'static [u8] {
&b"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"[..]
    }

    #[test]
    fn test_read() {
        assert_eq!(
            compute(Vec::from(test_data())),
            vec![6_000, 4_000, 11_000, 24_000, 10_000],
        )
    }

}