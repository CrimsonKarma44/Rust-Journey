// Encapsulation

pub struct AveragedCollection{
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection{
    pub fn new() -> AveragedCollection{
        AveragedCollection{list: Vec::new(), average: 0.0}
    }
    pub fn add(&mut self, value: i32){
        self.list.push(value);

        self.update_average();
    }
    pub fn pop(&mut self) -> Option<i32>{
        if let Some(value) = self.list.pop(){
    self.update_average();
            Some(value)
        }else {
            None
        }
    }

    pub fn average(&self) -> f64{
        self.average
    }
    fn update_average(&mut self){
        self.average = self.list.iter().sum::<i32>() as f64 / self.list.len() as f64;
    }
}


mod test {
    use crate::encapsulation::avgcol::AveragedCollection;

    #[test]
    fn test_average_collection(){
        let mut c = AveragedCollection::new();
        c.add(1);
        c.add(0);
        c.add(2);
        c.add(3);
        c.add(4);
        assert_eq!(c.average(), 2.0);
    }
    #[test]
    fn test_pop(){
        let mut c = AveragedCollection::new();
        c.add(1);
        c.add(2);
        c.add(3);
        c.add(4);
        assert_eq!(c.pop(), Some(4));
        assert_eq!(c.average(), 2.0);
        assert_ne!(c.average(), 5.0);
    }
}
