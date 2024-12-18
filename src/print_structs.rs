use uuid::Uuid;

#[derive(Debug)]
pub struct Spool {
    pub roll_id: Option<Uuid>,
    pub roll_name: Option<String>,
    pub roll_weight: Option<f32>,
    pub roll_length: Option<f32>,
    pub timestamp: Option<i64>,
}

#[derive(Debug)]
pub struct Filament {
    pub print_id: Option<Uuid>,
    pub print_weight: Option<f32>,
    pub print_length: Option<f32>,
    pub print_time: Option<i32>,
    pub roll_id: Option<Uuid>,
}

impl Spool {
    pub fn get_weight(&mut self) -> f32 {
        const CONVERSION_FACTOR: f32 = 3.0303;
        match self.roll_weight {
            Some(val) => val,
            None => {
                let length = match self.roll_length {
                    Some(val) => val as f32,
                    None => panic!("No Vals Set"),
                };
                let weight = length * CONVERSION_FACTOR;
                self.roll_weight = Some(weight);
                weight
            }
        }
    }

    pub fn get_length(&mut self) -> f32 {
        const CONVERSION_FACTOR: f32 = 0.33;
        match self.roll_length {
            Some(val) => val,
            None => {
                let weight = self.roll_weight.unwrap() as f32;
                let length = weight * CONVERSION_FACTOR;
                self.roll_length = Some(length);
                length
            }
        }
    }
}

impl Filament {
    pub fn get_weight(&mut self) -> f32 {
        const CONVERSION_FACTOR: f32 = 3.0303;
        match self.print_weight {
            Some(val) => val,
            None => {
                let length = self.print_length.unwrap();
                let weight = length * CONVERSION_FACTOR;
                self.print_weight = Some(weight);
                weight
            }
        }
    }

    pub fn get_length(&mut self) -> f32 {
        const CONVERSION_FACTOR: f32 = 0.33;
        match self.print_length {
            Some(val) => val,
            None => {
                let weight = self.print_weight.unwrap();
                let length = weight * CONVERSION_FACTOR;
                self.print_length = Some(length);
                length
            }
        }
    }
}
