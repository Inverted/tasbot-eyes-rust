use rand::Rng;

pub fn get_blink_delay(min_delay: u16, max_delay: u16, playback_speed: f32) -> u64 {
    if min_delay == max_delay {
        return ((min_delay as f32) * (1.0 / playback_speed)) as u64;
    }

    let mut rng = rand::thread_rng();
    let delay: u16 = rng.gen_range(min_delay..=max_delay);
    ((delay as f32) * (1.0 / playback_speed)) as u64 //return
}

pub fn get_blink_amount(max_blinks: u8) -> u8 {
    if max_blinks == 0 {
        return 0;
    }

    let mut rng = rand::thread_rng();
    rng.gen_range(1..=max_blinks) //return
}

#[cfg(test)]
mod tests {
    use super::*;

    //min_delay == max_delay
    #[test]
    fn test_get_blink_delay_min_delay_equals_max_delay() {
        let mut rng = rand::thread_rng();

        let delay = get_blink_delay(2000, 2000, 1.0);
        assert_eq!(delay, 2000);

        let delay = get_blink_delay(2000, 2000, 2.0);
        assert_eq!(delay, 1000);
    }

    //min_delay < max_delay
    #[test]
    fn test_get_blink_delay_min_delay_less_than_max_delay() {
        let mut rng = rand::thread_rng();

        let delay = get_blink_delay(1000, 2000, 1.0);
        assert!(delay >= 1000 && delay <= 2000);

        let delay = get_blink_delay(1000, 2000, 2.0);
        assert!(delay >= 500 && delay <= 1000);
    }
}