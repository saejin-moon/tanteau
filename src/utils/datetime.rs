pub struct DateTime {
    time: f64
}

pub enum TimeUnit {
    Second,
    Minute,
    Hour,
    Day,
    Month,
    Year
}

impl DateTime {
    pub fn new () -> Self {
        Self {
            time: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs_f64()
        }
    }
    pub fn time(&self) -> f64 {
        self.time
    }
    pub fn nano(&self) -> u128 {
        (self.time * 10_f64.powi(6)) as u128
    }
    pub fn millis(&self) -> u128 {
        (self.time * 10_f64.powi(3)) as u128
    }
    pub fn second(&self) -> u8 {
        (self.time % 60.) as u8
    }
    pub fn minute(&self) -> u8 {
        (self.time / 60. % 60.) as u8
    }
    // this is UTC! no time zone conversion done. can't do that without C code i don't want to get into or a dependency
    pub fn hour(&self) -> u8 {
        (self.time / 60. / 60. % 24.) as u8
    }
    pub fn day(&self) -> u8 {
        let (day, _, _) = self.date();
        day
    }
    pub fn month(&self) -> u8 {
        let (_, month, _) = self.date();
        month
    }
    pub fn year(&self) -> u16 {
        let (_, _, year) = self.date();
        year
    }
    fn is_leap(&self, year: u16) -> bool {
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }
    // this was an absolute pain to figure out
    pub fn date(&self) -> (u8, u8, u16) {
        // so this is days since epoch
        let mut day = (self.time / 60. / 60. / 24.) as u16;
        // years at epoch
        let mut year: u16 = 1970;
        
        // have to go through each year (cos leap year!) and subtract the days
        // this pinpoints year
        loop {
            let days_in_year = if self.is_leap(year) { 366 } else { 365 };
            if day < days_in_year { break; }
            day -= days_in_year;
            year += 1;
        }
        
        let mut days: [u16; 12] = [
            31, // january
            28, // february
            31, // march
            30, // april
            31, // may
            30, // june
            31, // july
            31, // august
            30, // september
            31, // october
            30, // november
            31  // december
        ];
        if self.is_leap(year) {
            days[1] = 29; // leap year! again.
        }
        // go through each month and subtract the days
        // this pinpoints the month and day
        for i in 0..days.len() {
            let month_day = days[i];
            if day < month_day {
                // (day, month, year)
                return ((day + 1) as u8, (i + 1) as u8, year);
            }
            else {
                day -= month_day;
            }
        }
        // just assume epoch
        // (day, month, year)
        (0, 0, 1970)
    }
    
    pub fn add(&mut self, unit: TimeUnit, val: u16) -> &mut Self {
        match unit {
            TimeUnit::Second => {
                self.time += val as f64;
            }
            TimeUnit::Minute => {
                self.time += (val * 60) as f64;
            }
            TimeUnit::Hour => {
                self.time += (val * 60 * 60) as f64;
            }
            TimeUnit::Day => {
                self.time += (val * 60 * 60 * 24) as f64;
            }
            /* 
            // yuck yuck muck
            TimeUnit::Month => {
                let mut month = val;

                
                let mut days: [u16; 12] = [
                    31, // january
                    28, // february
                    31, // march
                    30, // april
                    31, // may
                    30, // june
                    31, // july
                    31, // august
                    30, // september
                    31, // october
                    30, // november
                    31  // december
                ];
                if self.is_leap(self.year()) {
                    days[1] = 29; // leap year! again.
                }
                while month > 0 {
                    let index = ((self.month() - 1) as u16 + month) % 12;
                    self.time += days[index as usize] as f64 * 60. * 60. * 24.;
                    month -= 1;
                }
            }
            // less yuck but still muck
            TimeUnit::Year => {
                
            }
            */
        }
        self
    }
    pub fn sub(&mut self, unit: TimeUnit, val: u128) -> &mut Self {
        match unit {
            TimeUnit::Second => {
                self.time -= val as f64;
            }
            TimeUnit::Minute => {
                self.time -= (val * 60) as f64;
            }
            TimeUnit::Hour => {
                self.time -= (val * 60 * 60) as f64;
            }
            TimeUnit::Day => {
                self.time -= (val * 60 * 60 * 24) as f64;
            }
            /*
            TimeUnit::Month => {
                // self.time -= (val * )
            }
            TimeUnit::Year => {
                
            }
            */
        }
        self
    }
    /*
    pub fn floor(&mut self, unit: TimeUnit)
    pub fn round(&mut self, unit: TimeUnit)
    pub fn ceil(&mut self, unit: TimeUnit)
    */
}
