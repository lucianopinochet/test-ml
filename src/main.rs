use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct Model{
  food:f64,
  code:f64,
  sex:f64,
  friends:f64,
  sleep:f64,
}
#[derive(Debug, Deserialize)]
struct Training{
  food:bool,
  code:bool,
  sex:bool,
  friends:bool,
  sleep:bool,
  happiness:u8
}
fn main() {
  let mut rdr_training = csv::Reader::from_path("training.csv").unwrap();
  let mut rdr_model = csv::Reader::from_path("model.csv").unwrap();
  let mut model_result:[f64;5];
  let mut iter_outer = rdr_model.deserialize();
  if let Some(result_model) =  iter_outer.next(){
    let record_model:Model = result_model.unwrap();
    let mut iter_inner = rdr_training.deserialize();
    while let Some(result_training) = iter_inner.next(){
      let record_training:Training = result_training.unwrap();
      let mut total = 0.0;
      // let mut divider = 0;
      let training_list = [record_training.food, record_training.code, record_training.sex, record_training.friends, record_training.sleep];
      let model_list = [record_model.food, record_model.code, record_model.sex, record_model.friends, record_model.sleep];
      for (index, bool) in training_list.iter().enumerate(){
        if bool.clone(){
          total += model_list[index];
          // divider += 1;
        }
      }
      let mut percentage = [0.0, 0.0, 0.0, 0.0, 0.0];
      for (index, bool) in training_list.iter().enumerate(){
        if bool.clone(){
          percentage[index] = (model_list[index]/total) * record_training.happiness as f64;
        }
      }
      println!("{percentage:?}");
    }
  }

}
