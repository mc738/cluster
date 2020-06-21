use crate::job;

pub struct Request {
    pub request_type: RequestType
}

pub enum RequestType {
    Job(job::Job),
    Close(),
    Ping(String),
}

pub struct Response {
    pub body: String
}


impl Request {
    pub fn create_job(job: job::Job) -> Request {
        Request {
            request_type: RequestType::Job(job)
        }
    }

    pub fn create_close() -> Request {
        Request {
            request_type: RequestType::Close()
        }
    }

    pub fn create_ping(message: String) -> Request {
        Request {
            request_type: RequestType::Ping(message)
        }       
    }
}

impl Response {
    pub fn new(body: String) -> Response {
        Response { body }
    }
}