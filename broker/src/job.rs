use uuid::Uuid;

pub struct Job {
    pub id: uuid::Uuid,
    pub complete: bool,
    pub value: String,
    result: String,
}


impl Job {
    pub fn new(value: String) -> Job {
        Job {
            id: Uuid::new_v4(),
            complete: false,
            value: value,
            result: String::new(),
        }
    }

    pub fn get_result(&self) -> String {
        if self.complete {
            self.result.clone()
        } else {
            "Job not complete".to_string()
        }
    }
}
