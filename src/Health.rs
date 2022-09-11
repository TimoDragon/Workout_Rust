pub struct Health {

}

impl Health {
    pub fn compute_bmi(&self, weight: f32, height: f32) -> f32 {
        weight / (height * height)
    }

    pub fn bmi_string(&self, weight: f32, height: f32) -> String {
        let bmi: f32 = self.compute_bmi(weight, height);

        let mut s = String::new();

        if bmi <= 18.5 {
            s = String::from("untergewichtig");
        }
        else if bmi > 18.5 && bmi <= 25.0 {
            s = String::from("normalgewichtig");
        }
        else if bmi > 25.0 && bmi <= 30.0 {
            s = String::from("übergewichtig");
        }
        else {
            s = String::from("fettleibig");
        }

        String::from("Mit einem BMI von ".to_owned() + &bmi.to_string() + " sind Sie " + &s)
    }
}