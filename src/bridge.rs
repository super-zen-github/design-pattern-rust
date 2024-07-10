/// Bridge design pattern [structure]


pub trait MsgSender {
    fn send(&self, msg: String);
}

pub struct TelephoneMsgSender {
    telephones: Vec<String>,
}

pub struct EmailMsgSender {
    emails: Vec<String>,
}

impl TelephoneMsgSender {
    pub fn new(telephones: Vec<String>) -> Self {
        Self {
            telephones
        }
    }
}

impl EmailMsgSender {
    pub fn new(emails: Vec<String>) -> Self {
        Self {
            emails
        }
    }
}

impl MsgSender for TelephoneMsgSender {
    fn send(&self, msg: String) {
        println!("Sending telephone msg: {}, to: {:?}", msg, self.telephones);
    }
}

impl MsgSender for EmailMsgSender {
    fn send(&self, msg: String) {
        println!("Sending email msg: {}, to: {:?}", msg, self.emails);
    }
}

pub trait Notification {
    fn notify(&self, msg: String);
}

pub struct ServereNotification<'a> {
   msg_sender: &'a dyn MsgSender
}

pub struct NormalNotification<'a> {
   msg_sender: &'a dyn MsgSender
}

impl<'a> ServereNotification<'a> {
    pub fn new(msg_sender: &'a dyn MsgSender) -> Self {
        Self {
            msg_sender
        }
    }
}

impl<'a> NormalNotification<'a> {
    pub fn new(msg_sender: &'a dyn MsgSender) -> Self {
        Self {
            msg_sender
        }
    }
}

impl<'a> Notification for ServereNotification<'a> {
    fn notify(&self, msg: String) {
        self.msg_sender.send(msg);
    }
}

impl<'a> Notification for NormalNotification<'a> {
    fn notify(&self, msg: String) {
        self.msg_sender.send(msg);
    }
}
